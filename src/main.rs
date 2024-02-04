use aide::{
    axum::{routing::get, ApiRouter, IntoApiResponse},
    openapi::{Info, OpenApi},
    redoc::Redoc,
};
use axum::{http::StatusCode, Extension, Json};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = ApiRouter::new()
        .route("/docs", Redoc::new("/api.json").axum_route())
        .api_route("/alive", get(alive))
        .route("/api.json", get(docs))
        .layer(TraceLayer::new_for_http());

    let mut open_api = OpenApi {
        info: Info {
            title: String::from("Turn me off"),
            description: Some(String::from("An API to turn off devices")),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Could not add listen on port 3000");

    tracing::debug!(
        "Linstening on {}",
        listener
            .local_addr()
            .expect("We should be able to get the address on which we listen if we are listening")
    );

    axum::serve(
        listener,
        app.finish_api(&mut open_api)
            .layer(Extension(open_api))
            .into_make_service(),
    )
    .await
    .expect("This should run to the end of the program");
}

async fn docs(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}

/// A route to check if the turn-me-off server is alive
async fn alive() -> impl IntoApiResponse {
    (StatusCode::OK, "turn-me-off is alive")
}
