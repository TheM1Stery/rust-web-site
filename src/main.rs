use std::env;
use axum_web_test::endpoints::{serve, ServerOptions};
use axum_web_test::database::get_pooled_connection;
use sqlx::Result;
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

    let database_url = env::var("DATABASE_URL")?;
    let server_address = env::var("SERVER_ADDRESS")?;
    let server_port = env::var("SERVER_PORT")?;

    let pool = get_pooled_connection(&database_url)
        .await?;

    sqlx::migrate!()
        .run(&pool)
        .await?;


    tracing::info!("listening on {}", server_port);
    serve(ServerOptions {
        server_port: &server_port,
        server_address: &server_address,
        pool
    })
    .await;



    Ok(())
}
