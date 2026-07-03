use axum::body::Bytes;
use chrono::Utc;
use sqlx::Transaction;
use uuid::Uuid;

use crate::{
    db::mutations::works::{
        insert_new_edit, insert_new_poster, insert_new_script, insert_new_work,
        insert_new_work_credit,
    },
    errors::ApiError,
    models::{
        db::work::{Edit, Poster, Script, Work, WorkCategory},
        requests::works::{UploadEditReq, UploadPosterReq, UploadScriptReq},
    },
    services::json_extractor::AppJson,
};

pub async fn upload_work(
    data: Bytes,
    txn: &mut Transaction<'_, sqlx::Postgres>,
    user: Uuid,
    work_type: WorkCategory,
) -> Result<Uuid, ApiError> {
    match work_type {
        WorkCategory::EDIT => {
            let AppJson(data) = AppJson::<UploadEditReq>::from_bytes(&data)?;
            match &data.title {
                Some(title) => tracing::info!(work_title = %title, "Uploading edit"),
                None => tracing::info!("Uploading edit"),
            }
            let new_work = Work {
                id: Uuid::new_v4(),
                artist_id: user,
                title: data.title.map(|t| t.to_string()),
                stars: Some(0),
                created_at: Utc::now(),
                category: WorkCategory::EDIT,
            };
            let edit = Edit {
                work_id: new_work.id,
                src_id: data.src_id,
                platform: data.platform,
                format: data.format,
            };
            let new_work_id = insert_new_work(txn, new_work).await?;
            insert_new_edit(txn, edit).await?;
            if let Some(originals) = data.originals {
                for original in originals {
                    insert_new_work_credit(txn, original, new_work_id).await?;
                }
            }
            Ok(new_work_id)
        }
        WorkCategory::POSTER => {
            let AppJson(data) = AppJson::<UploadPosterReq>::from_bytes(&data)?;
            match &data.title {
                Some(title) => tracing::info!(work_title = %title, "Uploading poster"),
                None => tracing::info!("Uploading poster"),
            }

            let new_work = Work {
                id: Uuid::new_v4(),
                artist_id: user,
                title: data.title.map(|t| t.to_string()),
                stars: Some(0),
                created_at: Utc::now(),
                category: WorkCategory::POSTER,
            };
            let poster = Poster {
                work_id: new_work.id,
                src_id: data.src_id,
                format: data.format,
            };
            let new_work_id = insert_new_work(txn, new_work).await?;
            insert_new_poster(txn, poster).await?;
            if let Some(originals) = data.originals {
                for original in originals {
                    insert_new_work_credit(txn, original, new_work_id).await?;
                }
            }
            Ok(new_work_id)
        }
        WorkCategory::SCRIPT => {
            let AppJson(data) = AppJson::<UploadScriptReq>::from_bytes(&data)?;
            match &data.title {
                Some(title) => tracing::info!(work_title = %title, "Uploading script"),
                None => tracing::info!("Uploading script"),
            }
            let new_work = Work {
                id: Uuid::new_v4(),
                artist_id: user,
                title: data.title.map(|t| t.to_string()),
                stars: Some(0),
                created_at: Utc::now(),
                category: WorkCategory::SCRIPT,
            };
            let script = Script {
                work_id: new_work.id,
                img_src_ids: data.src_ids,
                thoughts: data.thoughts.iter().map(|t| t.to_string()).collect(),
            };

            let new_work_id = insert_new_work(txn, new_work).await?;
            insert_new_script(txn, script).await?;
            if let Some(originals) = data.originals {
                for original in originals {
                    insert_new_work_credit(txn, original, new_work_id).await?;
                }
            }
            Ok(new_work_id)
        }
    }
}
