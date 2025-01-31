use axum::{
    routing::{get, post},
    Router,
};

mod api;
mod global;
mod model;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().nest(
        "/api/v1",
        Router::new()
            .route("/ping", get(api::ping))
            .route("/ping", post(api::ping))
            .route("/echo", post(api::echo))
            .nest(
                "/room",
                Router::new()
                    .route("/create", post(api::create_room))
                    .route("/enter", post(api::enter_room))
                    .route("/start", post(api::start_room))
                    .route("/query", post(api::query_room)),
            ),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
