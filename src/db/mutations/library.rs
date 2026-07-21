use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::ApiError,
    models::{
        db::library::{LibraryEntry, LibraryEntryType, Recommendation, WatchlistStatus},
        requests::library::{
            TagWorkToLibraryEntryReq, UpdateLibraryEntryReq, UpdateRecommendationReq,
        },
    },
};

pub async fn insert_new_library_entry(pool: &PgPool, data: LibraryEntry) -> Result<Uuid, ApiError> {
    let mut txn = pool.begin().await?;

    // Read the peak BEFORE this entry can influence it, so we can freeze
    // it onto the row as peak_snapshot.
    let peak = sqlx::query_scalar!(
        "SELECT current_peak_library FROM profiles WHERE id = $1",
        data.profile_id
    )
    .fetch_one(&mut *txn)
    .await?;

    let new_entry = sqlx::query_as!(
        LibraryEntry,
        r#"
      INSERT INTO library (
          original_id,
          profile_id,
          pub_visibility,
          tagged_works,
          pre_thought,
          post_impression,
          status,
          entry_type,
          episode_id,
          id,
          created_at,
          surge_score,
          updated_at,
          peak_snapshot
        )
      VALUES (
          $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14
        )
        RETURNING original_id,
            profile_id,
            pub_visibility,
            tagged_works,
            pre_thought,
            post_impression,
            status as "status:WatchlistStatus",
            entry_type as "entry_type:LibraryEntryType",
            episode_id,
            id,
            created_at,
            surge_score,
            updated_at;
                "#,
        data.original_id,
        data.profile_id,
        data.pub_visibility,
        data.tagged_works.as_deref(),
        data.pre_thought,
        data.post_impression,
        data.status as WatchlistStatus,
        data.entry_type as LibraryEntryType,
        data.episode_id,
        data.id,
        data.created_at,
        data.surge_score,
        data.updated_at,
        peak
    )
    .fetch_one(&mut *txn)
    .await?;

    // Episode-level scoring isn't implemented yet — only originals-based
    // entries get folded into originals' running stats.
    if new_entry.original_id.is_none() {
        txn.commit().await?;
        return Ok(new_entry.id);
    }

    sqlx::query!(
        r#"
        WITH old_data AS (
            SELECT mean_surge, number_of_surges, surge_m2
            FROM originals WHERE id = $1
        ),
        calc AS (
            SELECT old_data.number_of_surges + 1 AS new_count,
                $2::float8 AS x,
                old_data.mean_surge AS old_mean,
                old_data.mean_surge + ($2::float8 - old_data.mean_surge)
                    / (old_data.number_of_surges + 1) AS new_mean,
                old_data.surge_m2 AS old_m2
            FROM old_data
        ),
        calc2 AS (
            SELECT calc.*, calc.old_m2 + (calc.x - calc.old_mean) * (calc.x - calc.new_mean) AS new_m2
            FROM calc
        )
        UPDATE originals
        SET number_of_surges = calc2.new_count,
            mean_surge = calc2.new_mean,
            surge_m2 = calc2.new_m2,
            surge_spread = CASE WHEN calc2.new_count > 1
                                 THEN sqrt(calc2.new_m2 / calc2.new_count)
                                 ELSE 0 END
        FROM calc2
        WHERE originals.id = $1
        "#,
        new_entry.original_id,
        new_entry.surge_score as f64
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        "UPDATE profiles SET current_peak_library = GREATEST(current_peak_library, $2) WHERE id = $1",
        data.profile_id,
        new_entry.surge_score
    )
    .execute(&mut *txn)
    .await?;

    txn.commit().await?;
    Ok(new_entry.id)
}

pub async fn insert_new_recommendation(
    pool: &PgPool,
    data: Recommendation,
) -> Result<Uuid, ApiError> {
    let res = sqlx::query_scalar!(
        r#"
        WITH inserted AS (
            INSERT INTO recommendations (
            id, original_id, artist_id, notes, created_at, updated_at,
                surge_score, boost_number, saves
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)
            RETURNING id, surge_score, artist_id
        )
        UPDATE profiles
            SET current_peak_recommendations = GREATEST(COALESCE(profiles.current_peak_recommendations, 1000), inserted.surge_score)
            FROM inserted
            WHERE profiles.id = inserted.artist_id
            RETURNING inserted.id
        "#,
        data.id,
        data.original_id,
        data.artist_id,
        data.notes,
        data.created_at,
        data.updated_at,
        data.surge_score,
        data.boost_number,
        data.saves
    )
    .fetch_one(pool)
    .await?;
    Ok(res)
}

pub async fn update_recommendation(
    pool: &PgPool,
    data: UpdateRecommendationReq,
    id: Uuid,
    artist_id: Uuid,
) -> Result<Uuid, ApiError> {
    let mut txn = pool.begin().await?;
    let res = sqlx::query_scalar!(
        r#"
        UPDATE recommendations
        SET
            notes = COALESCE($1, notes),
            surge_score = COALESCE($2, surge_score),
            updated_at = NOW()
        WHERE id = $3
        RETURNING id;
        "#,
        data.lines.as_ref().map(|l| l.to_string()),
        data.score,
        id
    )
    .fetch_one(&mut *txn)
    .await?;
    if let Some(_) = data.score {
        sqlx::query!(
            "
            WITH new_peak AS (
                SELECT surge_score
                FROM recommendations
                WHERE artist_id = $1
                ORDER BY surge_score DESC
                LIMIT 1
            )
            UPDATE profiles
            SET current_peak_recommendations = GREATEST(COALESCE(new_peak.surge_score, 1000), 1000)
            FROM new_peak
            WHERE profiles.id = $1
            AND profiles.current_peak_recommendations IS DISTINCT FROM GREATEST(COALESCE(new_peak.surge_score, 1000), 1000)
            ",
            artist_id
        )
        .execute(&mut *txn)
        .await?;
    }
    txn.commit().await?;
    Ok(res)
}

pub async fn update_library_entry(
    pool: &PgPool,
    data: UpdateLibraryEntryReq,
    id: Uuid,
) -> Result<Uuid, ApiError> {
    let mut txn = pool.begin().await?;

    let entry = sqlx::query!(
        r#"
        UPDATE library
        SET
            pre_thought = COALESCE($1, pre_thought),
            post_impression = COALESCE($2, post_impression),
            status = COALESCE($3, status),
            updated_at = NOW()
        WHERE id = $4
        RETURNING id, original_id, profile_id, surge_score, peak_snapshot;
        "#,
        data.pre_thought.as_ref().map(|t| t.to_string()),
        data.post_impression.as_ref().map(|t| t.to_string()),
        data.status as Option<WatchlistStatus>,
        id
    )
    .fetch_one(&mut *txn)
    .await?;

    // Episode-level scoring isn't implemented yet — skip originals update
    // for episode-type entries (original_id is NULL for those).
    let Some(original_id) = entry.original_id else {
        txn.commit().await?;
        return Ok(entry.id);
    };

    if let Some(new_score) = data.surge_score {
        let stats = sqlx::query!(
            r#"
            WITH peak AS (
                SELECT current_peak_library FROM profiles WHERE id = $1
            ),
            orig AS (
                SELECT mean_surge, surge_m2, number_of_surges
                FROM originals WHERE id = $2
            ),
            calc AS (
                SELECT
                    orig.number_of_surges AS n,
                    $3::float8 AS x_old,
                    $4::float8 AS x_new,
                    orig.mean_surge AS old_mean,
                    orig.surge_m2 AS old_m2,
                    peak.current_peak_library AS peak_now
                FROM orig, peak
            ),
            removed AS (
                SELECT calc.*,
                    CASE WHEN calc.n > 1
                         THEN (calc.n * calc.old_mean - calc.x_old) / (calc.n - 1)
                         ELSE 0 END AS mean_r,
                    CASE WHEN calc.n > 1
                         THEN calc.old_m2 - (calc.x_old - calc.old_mean) *
                              (calc.x_old - (calc.n * calc.old_mean - calc.x_old) / (calc.n - 1))
                         ELSE 0 END AS m2_r
                FROM calc
            ),
            final AS (
                SELECT removed.*,
                    removed.mean_r + (removed.x_new - removed.mean_r) / removed.n AS new_mean,
                    removed.m2_r + (removed.x_new - removed.mean_r) *
                        (removed.x_new - (removed.mean_r + (removed.x_new - removed.mean_r) / removed.n)) AS new_m2
                FROM removed
            )
            UPDATE originals
            SET mean_surge   = final.new_mean,
                surge_m2     = final.new_m2,
                surge_spread = CASE WHEN final.n > 1
                                    THEN sqrt(final.new_m2 / final.n)
                                    ELSE 0 END
            FROM final
            WHERE originals.id = $2
            RETURNING final.peak_now;
            "#,
            entry.profile_id,
            original_id,
            entry.surge_score as f64,
            new_score as f64
        )
        .fetch_one(&mut *txn)
        .await?;

        sqlx::query!(
            r#"
            UPDATE library
            SET surge_score = $1, peak_snapshot = $2
            WHERE id = $3
            "#,
            new_score,
            stats.peak_now,
            entry.id
        )
        .execute(&mut *txn)
        .await?;

        sqlx::query!(
            r#"
            WITH new_peak AS (
                SELECT MAX(surge_score) AS max_score
                FROM library
                WHERE profile_id = $1
            )
            UPDATE profiles
            SET current_peak_library = GREATEST(COALESCE(new_peak.max_score, 1000), 1000)
            FROM new_peak
            WHERE profiles.id = $1
            AND profiles.current_peak_library IS DISTINCT FROM GREATEST(COALESCE(new_peak.max_score, 1000), 1000)
            "#,
            entry.profile_id
        )
        .execute(&mut *txn)
        .await?;
    }

    txn.commit().await?;
    Ok(entry.id)
}

pub async fn delete_recommendation(
    pool: &PgPool,
    recommendation_id: Uuid,
    artist_id: Uuid,
) -> Result<(), ApiError> {
    sqlx::query!(
        r#"
        WITH deleted AS (
            DELETE FROM recommendations
            WHERE id = $1 AND artist_id = $2
            RETURNING artist_id
        ),
        new_peak AS (
            SELECT MAX(surge_score) AS max_score
            FROM recommendations
            WHERE artist_id = $2 AND id != $1
        )
        UPDATE profiles
            SET current_peak_recommendations = GREATEST(COALESCE(new_peak.max_score, 1000), 1000)
            FROM new_peak, deleted
            WHERE profiles.id = deleted.artist_id
            AND profiles.current_peak_recommendations IS DISTINCT FROM GREATEST(COALESCE(new_peak.max_score, 1000), 1000)
        "#,
        recommendation_id,
        artist_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn add_new_tagged_work(
    pool: &PgPool,
    data: TagWorkToLibraryEntryReq,
    entry_id: Uuid,
) -> Result<Uuid, ApiError> {
    Ok(sqlx::query_scalar!(
        "
      UPDATE library
      SET tagged_works = array_append(tagged_works, $1)
      WHERE id = $2
      RETURNING id;
      ",
        data.work_id,
        entry_id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn delete_library_entry(pool: &PgPool, entry_id: Uuid) -> Result<(), ApiError> {
    let mut txn = pool.begin().await?;

    let entry = sqlx::query!(
        r#"
        SELECT original_id, profile_id, surge_score, peak_snapshot
        FROM library
        WHERE id = $1
        "#,
        entry_id
    )
    .fetch_one(&mut *txn)
    .await?;

    sqlx::query!("DELETE FROM library WHERE id = $1", entry_id)
        .execute(&mut *txn)
        .await?;

    // Episode-level scoring isn't implemented yet — skip originals update
    // for episode-type entries.
    let Some(original_id) = entry.original_id else {
        txn.commit().await?;
        return Ok(());
    };

    sqlx::query!(
        r#"
        WITH orig AS (
            SELECT mean_surge, surge_m2, number_of_surges
            FROM originals WHERE id = $1
        ),
        calc AS (
            SELECT
                orig.number_of_surges AS n,
                $2::float8 AS x_old,
                orig.mean_surge AS old_mean,
                orig.surge_m2 AS old_m2
            FROM orig
        ),
        final AS (
            SELECT
                calc.*,
                GREATEST(calc.n - 1, 0) AS new_count,
                CASE WHEN calc.n > 1
                     THEN (calc.n * calc.old_mean - calc.x_old) / (calc.n - 1)
                     ELSE 0 END AS new_mean,
                CASE WHEN calc.n > 1
                     THEN calc.old_m2 - (calc.x_old - calc.old_mean) *
                          (calc.x_old - (calc.n * calc.old_mean - calc.x_old) / (calc.n - 1))
                     ELSE 0 END AS new_m2
            FROM calc
        )
        UPDATE originals
        SET number_of_surges = final.new_count,
            mean_surge       = final.new_mean,
            surge_m2         = final.new_m2,
            surge_spread     = CASE WHEN final.new_count > 1
                                    THEN sqrt(final.new_m2 / final.new_count)
                                    ELSE 0 END
        FROM final
        WHERE originals.id = $1
        "#,
        original_id,
        entry.surge_score as f64
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        r#"
        WITH current_peak AS (
            SELECT MAX(surge_score) AS max_score
            FROM library
            WHERE profile_id = $1
        )
        UPDATE profiles
        SET current_peak_library = GREATEST(COALESCE(current_peak.max_score, 0), 1000)
        FROM current_peak
        WHERE profiles.id = $1
        AND profiles.current_peak_library
            IS DISTINCT FROM GREATEST(COALESCE(current_peak.max_score, 0), 1000)
        "#,
        entry.profile_id
    )
    .execute(&mut *txn)
    .await?;

    txn.commit().await?;
    Ok(())
}
