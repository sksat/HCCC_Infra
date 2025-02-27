use axum::{
    extract::{self, Extension, Path, Query},
    routing,
    Json,
    Router,
};
use serde::Deserialize;

use crate::services;
use crate::entities::{Submission, UserSubmissions};
use crate::request::UserContext;
use crate::database::RepositoryProvider;
use crate::is_contest_underway;

pub fn submissions() -> Router {
    Router::new()
        .route("/", routing::get(from_user_id))
        .route("/:id", routing::get(from_submit_id))
}

async fn from_user_id(
    Query(param): Query<UserIdParam>,
    user_context: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> Json<UserSubmissions> {
    tracing::debug!("/api/submissions");
    let submission_repo = repository_provider.submission();
    if let Some(user_id) = param.user_id {
        if !is_contest_underway() && user_context.user_id() != user_id {
            Json(UserSubmissions::error("forbidden", "You won't be able to see other users' submissions during the contest"))
        } else {
            Json(services::get_user_submissions(&submission_repo, user_id).await)
        }
    } else {
        if !is_contest_underway() {
            Json(UserSubmissions::error("forbidden", "You won't be able to see other users' submissions during the contest"))
        } else {
            Json(services::get_all_users_submissions(&submission_repo).await)
        }
    }
}

async fn from_submit_id(
    Path(id): Path<i32>,
    _: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> Json<Submission> {
    tracing::debug!("/api/submissions/:id");
    let submission_repo = repository_provider.submission();
    Json(services::get_submission(&submission_repo, id).await)
}

pub async fn submit(
    Path(id): Path<i32>,
    extract::Json(req): extract::Json<SubmitReq>,
    user_context: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> Json<Submission> {
    tracing::debug!("/api/problems/:id/submissions");
    let user_repo = repository_provider.user();
    let problem_repo = repository_provider.problem();
    let submission_repo = repository_provider.submission();
    Json(
        services::submit_asm(
            &user_repo,
            &problem_repo,
            &submission_repo,
            user_context.user_id(),
            id,
            req.asm,
        ).await
    )
}

#[derive(Deserialize)]
pub struct SubmitReq {
    asm: String,
}

#[derive(Deserialize)]
struct UserIdParam {
    user_id: Option<i32>,
}
