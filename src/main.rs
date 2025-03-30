use axum::{
    routing::{get, put},
    Router,
};
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            api::alive,
            api::turn_off,
            api::reboot,
            api::suspend,
            api::hostname,
        ),
        tags(
            (
                name="turn-me-off",
                description="HTTP API to turn off and reboot the device on which it is deployed",
            )
        )
    )]
    struct ApiDoc;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "turn_me_off=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .route("/alive", get(api::alive))
        .route("/turn-off", put(api::turn_off))
        .route("/reboot", put(api::reboot))
        .route("/suspend", put(api::suspend))
        .route("/hostname", get(api::hostname))
        .layer(TraceLayer::new_for_http());

    let port = std::env::var("TMF_PORT").unwrap_or(String::from("3000"));
    let full_url = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(full_url)
        .await
        .unwrap_or_else(|_| panic!("Could not listen on port {port}"));

    tracing::info!(
        "Listening on {}",
        listener
            .local_addr()
            .expect("We should be able to get the address on which we listen if we are listening")
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown())
        .await
        .expect("This should run to the end of the program");
}

/// Handles graceful shutdown for Ctrl+c and SIGTERM signals.
///
/// Note that this only works on unix based systems.
async fn shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+c listener");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM listener")
            .recv()
            .await
    };

    tokio::select! {
        () = ctrl_c => { tracing::info!("Ctrl+c received. Bye!")},
        _ = terminate => { tracing::info!("SIGTERM received. Bye!")},
    }
}
