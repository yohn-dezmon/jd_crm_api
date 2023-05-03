
use jd_crm_api::run;
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_uri = env::var("DATABASE_URL").expect(
        "DATABASE_URL env var is required for connecting to the db and for sqlx macros"
    );
    run(&db_uri).await
}
