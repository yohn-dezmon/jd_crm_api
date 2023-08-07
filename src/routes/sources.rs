use crate::helpers::handler_utils::build_link_tables;
use crate::helpers::shared_types::{CreateSource, ImageType, MediaType};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Source {
    id: i32,
    name: Option<String>,
    url: Option<String>,
    author: Option<String>,
    author_url: Option<String>,
    media_type: Option<MediaType>,
    image_url: Option<String>,
    image_type: Option<ImageType>,
    ai_generated: Option<bool>,
}

#[derive(Deserialize)]
pub struct GetSourceQueryParams {
    id: i32,
}

/*
 /sources
- returns all sources
 */
pub async fn get_all_sources_handler(State(db_pool): State<PgPool>) -> Response {
    let sources = get_all_sources(&db_pool).await;
    match sources {
        Ok(sources) => (StatusCode::OK, Json(sources)).into_response(),
        // for errors Axum expects the axum::response::Response type
        // example output: error returned from database: relation "platform.tipics" does not exist
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}

pub async fn get_all_sources(db_pool: &PgPool) -> Result<Vec<Source>> {
    let sources = sqlx::query_as::<_, Source>(
        "SELECT id,
        name,
        url,
        author,
        author_url,
        media_type,
        image_url,
        image_type,
        ai_generated
    FROM platform.sources",
    )
    .fetch_all(db_pool)
    .await?;
    Ok(sources)
}

/*
/new-source
Body:
{
   "value": "<new_source_name>"
}
*/
pub async fn new_source_handler(
    State(db_pool): State<PgPool>,
    Json(payload): Json<CreateSource>,
) -> Response {
    let insert_result = insert_source(&payload, &db_pool).await;
    let link_insert_result = build_link_tables(&payload, "source", &db_pool).await;
    match (insert_result, link_insert_result) {
        (Ok(_), Ok(_)) => "new source created".into_response(),
        (Err(error), _) | (_, Err(error)) => {
            (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
        }
    }
}

pub async fn insert_source(payload: &CreateSource, db_pool: &PgPool) -> Result<()> {
    let _insert_result = sqlx::query(
        "
                INSERT INTO platform.sources 
                    (name,
                    url,
                    author,
                    author_url,
                    media_type,
                    image_url,
                    image_type,
                    ai_generated) 
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(&payload.name)
    .bind(&payload.url)
    .bind(&payload.author)
    .bind(&payload.author_url)
    .bind(&payload.media_type)
    .bind(&payload.image_url)
    .bind(&payload.image_type)
    .bind(&payload.ai_generated)
    .execute(db_pool)
    .await;

    Ok(())
}

pub async fn get_source_handler(
    State(db_pool): State<PgPool>,
    params: axum::extract::Query<GetSourceQueryParams>,
) -> Response {
    let source = get_source(&db_pool, &params.id).await;
    match source {
        Ok(source) => (StatusCode::OK, Json(source)).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}

pub async fn get_source(db_pool: &PgPool, id: &i32) -> Result<Source> {
    let source = sqlx::query_as::<_, Source>("SELECT * from platform.sources where id = $1")
        .bind(id)
        .fetch_one(db_pool)
        .await?;
    Ok(source)
}
