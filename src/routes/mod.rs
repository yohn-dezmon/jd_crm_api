/*
This file creates the routes.
*/

mod hello_world;
mod links;
mod sources;
mod terms;
mod topics;
use axum::http::Method;
use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use hello_world::hello_world;
use links::new_link_handler;
use sources::{get_all_sources_handler, get_source_handler, new_source_handler};
use sqlx::postgres::PgPool;
use terms::{
    get_all_terms_for_topic_handler, get_all_terms_handler, get_term_handler, new_term_handler,
};
use topics::{get_all_topics_handler, get_topic_handler, new_topic_handler};
use tower_http::cors::{Any, CorsLayer};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db_pool: PgPool,
}

pub fn create_routes(db_pool: PgPool) -> Router {
    let app_state: AppState = AppState { db_pool };

    // Cors settings for all routes
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/", get(hello_world))
        .route("/topics", get(get_all_topics_handler))
        .route("/topic", get(get_topic_handler))
        .route("/terms", get(get_all_terms_handler))
        .route("/term", get(get_term_handler))
        .route("/terms-from-topic", get(get_all_terms_for_topic_handler))
        .route("/new-topic", post(new_topic_handler))
        .route("/new-term", post(new_term_handler))
        .route("/sources", get(get_all_sources_handler))
        .route("/new-source", post(new_source_handler))
        .route("/source", get(get_source_handler))
        .route("/link-entities", post(new_link_handler))
        .layer(cors)
        .with_state(app_state)
}
