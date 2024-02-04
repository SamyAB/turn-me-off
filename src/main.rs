use axum::{http::StatusCode, routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            alive,
        ),
        tags(
            (name="turn-me-off", description="HTTP API to turn off the device on which it is deployed")
        )
    )]
    struct ApiDoc;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/alive", get(alive))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Could not add listen on port 3000");

    tracing::debug!(
        "Linstening on {}",
        listener
            .local_addr()
            .expect("We should be able to get the address on which we listen if we are listening")
    );

    axum::serve(listener, app)
        .await
        .expect("This should run to the end of the program");
}

/// A route to check if the turn-me-off server is alive
#[utoipa::path(get, path = "/alive")]
async fn alive() -> (StatusCode, &'static str) {
    (StatusCode::OK, "turn-me-off is alive")
}
