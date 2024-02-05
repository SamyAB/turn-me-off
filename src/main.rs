use axum::{
    routing::{get, put},
    Router,
};
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            api::alive,
            api::turn_off,
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
        .route("/alive", get(api::alive))
        .route("/turn-off", put(api::turn_off))
        .layer(TraceLayer::new_for_http());

    let port = std::env::var("TMF_PORT").unwrap_or(String::from("3000"));
    let full_url = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(full_url)
        .await
        .unwrap_or_else(|_| panic!("Could not listen on port {port}"));

    tracing::debug!(
        "Linstening on {}",
        listener
            .local_addr()
            .expect("We should be able to get the address on which we listen if we are listening")
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown())
        .await
        .expect("This should run to the end of the program");
}

mod api {
    use axum::http::StatusCode;

    /// Checks if the turn-me-off server is alive.
    #[utoipa::path(get, path = "/alive", responses((status = 200, body = String, description = "Alive message")))]
    pub async fn alive() -> (StatusCode, &'static str) {
        (StatusCode::OK, "turn-me-off is alive")
    }

    /// Turns off the machine on which this HTTP server runs.
    #[utoipa::path(put, path = "/turn-off", responses((status = 200, body = String, description = "Turn off message")))]
    pub async fn turn_off() -> (StatusCode, &'static str) {
        tokio::process::Command::new("/usr/bin/systemctl")
            .arg("poweroff")
            .spawn()
            .expect("I should be able to run the command");
        (StatusCode::OK, "This machine will now turn off.")
    }
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
        () = ctrl_c => { tracing::debug!("Ctrl+c received. Bye!")},
        _ = terminate => { tracing::debug!("SIGTERM received. Bye!")},
    }
}
