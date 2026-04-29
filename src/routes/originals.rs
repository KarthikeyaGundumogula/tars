use std::sync::Arc;

use axum::{Json, extract::State};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    db::originals::{insert_new_original, insert_new_role},
    errors::ApiError,
    types::{
        db::{
            original::Original,
            profile::{Role, RoleType},
        },
        requests::originals::CreateOriginalReq,
        response::ApiResponse,
    },
    utils::password::get_password_hash,
};

pub async fn create_new_original_handler(
    State(pool): State<Arc<PgPool>>,
    Json(data): Json<CreateOriginalReq>,
) -> Result<ApiResponse, ApiError> {
    let password_hash = get_password_hash(&data.password)?;
    let mut txn = pool.begin().await?;
    let orignal_data = Original {
        id: Uuid::new_v4(),
        title: data.title,
        release_date: data.release_date,
        description: data.description,
        cover_img: data.cover_img,
        presence: 100,
        password_hash,
        associated_with: data.associated_with,
        generes: data.genere,
        created_at: Utc::now(),
    };
    let original_id = insert_new_original(&mut txn, orignal_data).await?;
    for star in data.stars.iter() {
        let role = Role {
            profile_id: star.artist,
            category: RoleType::STAR,
            original_id,
            role_name: star.role.clone(),
            created_at: Utc::now(),
        };
        insert_new_role(&mut txn, role).await?;
    }
    for maker in data.makers.iter() {
        let role = Role {
            profile_id: maker.artist,
            category: RoleType::MAKER,
            original_id,
            role_name: maker.role.clone(),
            created_at: Utc::now(),
        };
        insert_new_role(&mut txn, role).await?;
    }
    txn.commit().await?;
    Ok(ApiResponse::OK)
}
