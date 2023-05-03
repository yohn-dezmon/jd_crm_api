use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, FromRow, PgPool, Result};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Topic {
    id: i32,
    topic: String,
    brief_description: Option<String>,
    full_description: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateTopic {
    topic: String,
}

/*
 /topics
- returns all topics
 */
pub async fn get_all_topics_handler(State(db_pool): State<PgPool>) -> Response {
    let topics = get_all_topics(&db_pool).await;
    match topics {
        Ok(topics) => (StatusCode::OK, Json(topics)).into_response(),
        // for errors Axum expects the axum::response::Response type
        // example output: error returned from database: relation "platform.tipics" does not exist
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
    }
}

pub async fn get_all_topics(db_pool: &PgPool) -> Result<Vec<Topic>> {
    let topics = sqlx::query_as::<_, Topic>("SELECT id, topic, brief_description, full_description FROM platform.topics")
        .fetch_all(db_pool)
        .await?;
    Ok(topics)
}

/*
/new-topic
Body:
{
   "topic": "<new_topic_name>"
}
*/
pub async fn new_topic_handler(
    State(db_pool): State<PgPool>,
    Json(payload): Json<CreateTopic>,
) -> Response {
    // TODO: add definition to topic creation!
    let topic = &payload.topic;
    let insert_result = query!("INSERT INTO platform.topics (topic) VALUES ($1)", topic)
        .execute(&db_pool)
        .await;
    match insert_result {
        Ok(_insert_result) => "new topic created".into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}
