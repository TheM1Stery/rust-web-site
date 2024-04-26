use axum::{extract::Request, routing::get, Router, ServiceExt};
use sqlx::SqlitePool;
use tokio::signal;
use tower_http::{normalize_path::NormalizePathLayer, trace::TraceLayer};
use tower_layer::Layer;
use user::user_router;

use self::misc::{healthcheck, index, test, test2};

mod errors;
mod misc;
mod user;

#[derive(Clone)]
struct AppState {
    pub db_pool: SqlitePool,
}

pub struct ServerOptions<'a> {
    pub server_port: &'a str,
    pub server_address: &'a str,
    pub pool: SqlitePool,
}

pub async fn serve(options: ServerOptions<'_>) {
    let state = AppState {
        db_pool: options.pool,
    };

    let router = routes().layer(TraceLayer::new_for_http()).with_state(state);

    let app = NormalizePathLayer::trim_trailing_slash().layer(router);

    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        options.server_address, options.server_port
    ))
    .await
    .unwrap();

    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(index))
        .route("/dummy_healthcheck", get(healthcheck))
        .route("/second", get(test))
        .route("/third", get(test2))
        .nest("/user", user_router())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install ctrl_c handler");
    };

    #[cfg(unix)]
    let terminate = async {
        use signal::unix::{signal, SignalKind};
        signal(SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {}
    }
}
