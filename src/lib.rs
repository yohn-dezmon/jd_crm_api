mod routes;
mod helpers;
use axum::Router;
use routes::create_routes;
use sqlx::postgres::PgPoolOptions;

pub async fn run(db_uri: &str) {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_uri)
        .await
        .expect("db pool failed to initialize");

    // build our server/application
    let app: Router = create_routes(pool);

    // run it with hyper on localhost:3000
    // 0.0.0.0 makes it compatible with docker containers
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .expect("server failed to start");
}
