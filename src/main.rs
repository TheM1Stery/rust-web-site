use std::env;
use axum::routing::post;
use axum::{routing::get, Router};
use axum_web_test::{create_user, get_user, healthcheck, index, return_json, AppState};
use axum_web_test::database::get_pooled_connection;
use sqlx::Result;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "axum_web_test=debug,tower_http=debug,axum::rejection=trace,sqlx=debug".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();


    let router = Router::new()
                    .route("/", get(index))
                    .route("/dummy_healthcheck", get(healthcheck))
                    .route("/user", post(create_user))
                    .route("/user/:id", get(get_user))
                    .layer(TraceLayer::new_for_http());

    let database_url = env::var("DATABASE_URL")?;
    let server_address = env::var("SERVER_ADDRESS")?;
    let server_port = env::var("SERVER_PORT")?;

    let pool = get_pooled_connection(&database_url)
        .await?;

    sqlx::migrate!()
        .run(&pool)
        .await?;

    let router = router.with_state(AppState {
        db_pool: pool
    });


    let listener = tokio::net::TcpListener::bind(format!("{server_address}:{server_port}"))
        .await
        .unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
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
