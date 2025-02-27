use axum::{
    extract::{Extension, Path},
    routing,
    Json,
    Router,
};

use crate::services;
use crate::entities::{Problem, AllProblems};
use crate::request::UserContext;
use crate::database::RepositoryProvider;
use crate::controllers::submissions::submit;
use crate::is_contest_underway;

pub fn problem() -> Router {
    Router::new()
        .route("/", routing::get(all_problem))
        .route("/:id", routing::get(problem_from_id))
        .route("/:id/submissions", routing::post(submit))
}

async fn all_problem(
    _: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> Json<AllProblems> {
    tracing::debug!("/api/problems");
    if is_contest_underway() {
        let problem_repo = repository_provider.problem();
        Json(services::get_all_problems(&problem_repo).await)
    } else {
        Json(AllProblems::error("forbidden", "problems has not been opened yet"))
    }
}

async fn problem_from_id(
    Path(id): Path<i32>,
    _: UserContext,
    Extension(repository_provider): Extension<RepositoryProvider>
) -> Json<Problem> {
    tracing::debug!("/api/problems/:id");
    if is_contest_underway() {
        let problem_repo = repository_provider.problem();
        Json(services::get_problem(&problem_repo, id).await)
    } else {
        Json(Problem::error("forbidden", "problems has not been opened yet"))
    }
}
