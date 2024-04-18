use axum::Router;
use sqlx::SqlitePool;
use tokio::signal;
use tower_http::trace::TraceLayer;
use user::user_router;

mod user;
mod errors;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: SqlitePool,
}

pub struct ServerOptions<'a> {
    server_port: &'a str,
    server_address: &'a str,
    pool: SqlitePool
}

pub async fn serve(options: ServerOptions<'_>) {
    // let state = AppState {
    //     db_pool: todo!(),
    // };
    //

    let router = routes();


    let listener = tokio::net::TcpListener::bind(format!("{}:{}", options.server_address, options.server_port))
        .await
        .unwrap();


    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}


fn routes() -> Router {
}

async fn shutdown_signal(){
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

