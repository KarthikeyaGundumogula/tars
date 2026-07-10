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
          updated_at
        )
      VALUES (
          $1,
          $2,
          $3,
          $4,
          $5,
          $6,
          $7,
          $8,
          $9,
          $10,
          $11,
          $12,
          $13
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
        data.updated_at
    )
    .fetch_one(&mut *txn)
    .await?;
    sqlx::query!(
        r#"
            WITH profile_data AS (
        SELECT current_peak_library
        FROM profiles
        WHERE id = $1
        ),
        old_data AS (
        SELECT mean_surge,
            number_of_surges,
            surge_m2
        FROM originals
        WHERE id = $2
        ),
        calc AS (
        SELECT old_data.number_of_surges + 1 AS new_count,
            ($3 / profile_data.current_peak_library) AS x,
            old_data.mean_surge AS old_mean,
            old_data.mean_surge + (
            ($3 / profile_data.current_peak_library) - old_data.mean_surge
            ) / (old_data.number_of_surges + 1) AS new_mean,
            old_data.surge_m2 AS old_m2
        FROM old_data,
            profile_data
            ),
            calc2 AS (
            SELECT calc.*,
                calc.old_m2 + (calc.x - calc.old_mean) * (calc.x - calc.new_mean) AS new_m2
            FROM calc
        )
        UPDATE originals
        SET number_of_surges = calc2.new_count,
        mean_surge = calc2.new_mean,
        surge_m2 = calc2.new_m2,
        surge_spread = CASE
            WHEN calc2.new_count > 1 THEN sqrt(calc2.new_m2 / (calc2.new_count - 1))
            ELSE 0
        END
        FROM calc2
        WHERE originals.id = $2
        "#,
        data.profile_id,
        data.original_id,
        data.surge_score
    )
    .execute(&mut *txn)
    .await?;

    sqlx::query!(
        r#"
    UPDATE profiles
    SET current_peak_library = GREATEST(current_peak_library, $2)
    WHERE id = $1
    "#,
        data.profile_id,
        data.surge_score
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
    let mut txn = pool.begin().await?;
    let res = sqlx::query_scalar!(
        r#"
        INSERT INTO recommendations (
        id,
            original_id,
            artist_id,
            notes,
            created_at,
            updated_at,
            surge_score,
            boost_number,
            saves
        ) VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7,
            $8,
            $9
        ) RETURNING id;
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
    .fetch_one(&mut *txn)
    .await?;
    sqlx::query!(
        "
        UPDATE profiles
        SET current_peak_recommendations = GREATEST(current_peak_recommendations,$2)
        WHERE id = $1
        ",
        data.artist_id,
        data.surge_score
    )
    .execute(&mut *txn)
    .await?;
    txn.commit().await?;
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
            WITH current_peak AS (
                SELECT surge_score 
                FROM recommendations 
                WHERE artist_id = $1
                ORDER BY surge_score DESC
                LIMIT 1
            )
            UPDATE profiles
            SET current_peak_recommendations = GREATEST(current_peak.surge_score,1000)
            FROM current_peak
            WHERE profiles.id = $1
            AND
            profiles.current_peak_recommendations IS DISTINCT FROM GREATEST(current_peak.surge_score, 1000)
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
    Ok(sqlx::query_scalar!(
        r#"
      UPDATE library
      SET
          pre_thought = COALESCE($1, pre_thought),
          post_impression = COALESCE($2, post_impression),
          status = COALESCE($3,status),
          updated_at = NOW()
      WHERE id = $4
      RETURNING id;
      "#,
        data.pre_thought.as_ref().map(|t| t.to_string()),
        data.post_impression.as_ref().map(|t| t.to_string()),
        data.status as Option<WatchlistStatus>,
        id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn add_new_tagged_work(
    pool: &PgPool,
    data: TagWorkToLibraryEntryReq,
    entry_id: Uuid,
) -> Result<Uuid, ApiError> {
    Ok(sqlx::query_scalar!(
        "
      Update library
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
    sqlx::query!(
        "
      DELETE FROM library
      WHERE id = $1
      ",
        entry_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
