use crate::helpers::handler_utils::{build_link_tables, insert_topic_or_term, CreateTopicOrTerm};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, FromRow, PgPool, Result};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Term {
    id: i32,
    term: String,
    is_verified: bool,
    brief_description: Option<String>,
    full_description: Option<String>,
    bullet_points: Option<Vec<String>>,
    examples: Option<Vec<String>>,
    parallels: Option<Vec<String>>,
    ai_brief_description: Option<String>,
    ai_full_description: Option<String>,
    ai_bullet_points: Option<Vec<String>>,
    ai_parallels: Option<Vec<String>>,
    ai_examples: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct AllTermsQueryParams {
    topic: String,
}

#[derive(Deserialize)]
pub struct GetTermQueryParams {
    id: i32,
}

pub async fn get_all_terms_handler(State(db_pool): State<PgPool>) -> Response {
    let terms = get_all_terms(&db_pool).await;
    match terms {
        Ok(terms) => (StatusCode::OK, Json(terms)).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}

pub async fn get_all_terms(db_pool: &PgPool) -> Result<Vec<Term>> {
    let terms = sqlx::query_as::<_, Term>(
        "SELECT id, term, is_verified, brief_description,
    full_description, bullet_points, examples, parallels, ai_brief_description, ai_full_description,
    ai_bullet_points, ai_parallels, ai_examples
    FROM platform.terms",
    )
    .fetch_all(db_pool)
    .await?;
    Ok(terms)
}

/*
Ex1:
http://localhost:3000/terms-from-topic?topic=new%20topic
 */
pub async fn get_all_terms_for_topic_handler(
    State(db_pool): State<PgPool>,
    params: axum::extract::Query<AllTermsQueryParams>,
) -> Response {
    let terms = get_all_terms_for_a_topic(&db_pool, &params.topic).await;

    match terms {
        Ok(terms) => (StatusCode::OK, Json(terms)).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}

pub async fn get_all_terms_for_a_topic(db_pool: &PgPool, topic: &str) -> Result<Vec<Term>> {
    // first get topic id
    let record = query!("SELECT id from platform.topics where topic = $1", topic)
        .fetch_one(db_pool)
        .await?;

    let terms: Vec<Term> = sqlx::query_as!(
        Term,
        "SELECT id, term, is_verified, brief_description,
        full_description, bullet_points, examples, parallels, ai_brief_description, ai_full_description,
        ai_bullet_points, ai_parallels, ai_examples FROM platform.terms as terms 
        INNER JOIN platform.terms_to_topics as terms_to_topics on 
        terms.id = terms_to_topics.term_id 
        where terms_to_topics.topic_id = $1",
        record.id
    )
    .fetch_all(db_pool)
    .await?;

    Ok(terms)
}

/*
/new-topic
Body:
{
   "topic": "<new_topic_name>"
}
*/
pub async fn new_term_handler(
    State(db_pool): State<PgPool>,
    Json(payload): Json<CreateTopicOrTerm>,
) -> Response {
    let insert_result = insert_topic_or_term(&payload, "term", &db_pool).await;
    let link_insert_result = build_link_tables(&payload, "term", &db_pool).await;
    match (insert_result, link_insert_result) {
        (Ok(_), Ok(_)) => "new term created".into_response(),
        (Err(error), _) | (_, Err(error)) => {
            (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
        }
    }
}

pub async fn get_term_handler(
    State(db_pool): State<PgPool>,
    params: axum::extract::Query<GetTermQueryParams>,
) -> Response {
    let term = get_term(&db_pool, &params.id).await;
    match term {
        Ok(term) => (StatusCode::OK, Json(term)).into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response(),
    }
}

pub async fn get_term(db_pool: &PgPool, id: &i32) -> Result<Term> {
    let term = sqlx::query_as!(Term, "SELECT * from platform.terms where id = $1", id)
        .fetch_one(db_pool)
        .await?;
    Ok(term)
}
