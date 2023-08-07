use crate::helpers::handler_utils::update_link_table;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use sqlx::{FromRow, PgPool};

#[derive(Deserialize, FromRow)]
pub struct CreateLink {
    parent_entity_type: String,
    child_entity_type: String,
    parent_id: i32,
    related_term_ids: Option<Vec<i32>>,
    related_topic_ids: Option<Vec<i32>>,
    related_source_ids: Option<Vec<i32>>,
}

pub async fn new_link_handler(
    State(db_pool): State<PgPool>,
    Json(payload): Json<CreateLink>,
) -> Response {
    let mut terms_insert_result = Some(Ok(()));
    let mut topics_insert_result = Some(Ok(()));
    let mut sources_insert_result = Some(Ok(()));

    if let Some(related_term_ids) = &payload.related_term_ids {
        terms_insert_result = Some(
            update_link_table(
                &payload.parent_entity_type,
                &payload.child_entity_type,
                &payload.parent_id,
                related_term_ids,
                &db_pool,
            )
            .await,
        );
    }
    if let Some(related_topic_ids) = &payload.related_topic_ids {
        topics_insert_result = Some(
            update_link_table(
                &payload.parent_entity_type,
                &payload.child_entity_type,
                &payload.parent_id,
                related_topic_ids,
                &db_pool,
            )
            .await,
        );
    }
    if let Some(related_source_ids) = &payload.related_source_ids {
        sources_insert_result = Some(
            update_link_table(
                &payload.parent_entity_type,
                &payload.child_entity_type,
                &payload.parent_id,
                related_source_ids,
                &db_pool,
            )
            .await,
        );
    }

    match (
        terms_insert_result,
        topics_insert_result,
        sources_insert_result,
    ) {
        (Some(Ok(_)), Some(Ok(_)), Some(Ok(_))) => "new link created".into_response(),
        (Some(Err(error)), _, _) | (_, Some(Err(error)), _) | (_, _, Some(Err(error))) => {
            (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
        }
        _ => {
            // no updates made
            (StatusCode::BAD_REQUEST, "no related entities provided").into_response()
        }
    }
}
