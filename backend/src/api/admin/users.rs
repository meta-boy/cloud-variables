use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::dto::{UpdateUserRequest, UserManagementResponse, UserQueryParams};
use crate::error::Result;
use crate::repositories::UserRepository;

pub async fn list_users(
    State(pool): State<Pool<Postgres>>,
    Query(params): Query<UserQueryParams>,
) -> Result<Json<UserManagementResponse>> {
    let user_repo = UserRepository::new(pool);

    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).min(100).max(1);

    let (users, total) = user_repo
        .list(page, page_size, params.search.as_deref())
        .await?;

    let public_users = users.into_iter().map(|u| u.sanitize()).collect();

    Ok(Json(UserManagementResponse {
        users: public_users,
        total,
        page,
        page_size,
    }))
}

pub async fn update_user(
    State(pool): State<Pool<Postgres>>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<StatusCode> {
    let user_repo = UserRepository::new(pool);

    if let Some(is_active) = payload.is_active {
        user_repo.update_status(user_id, is_active).await?;
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_user(
    State(pool): State<Pool<Postgres>>,
    Path(user_id): Path<Uuid>,
) -> Result<StatusCode> {
    let user_repo = UserRepository::new(pool);
    user_repo.delete(user_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
