use axum::{response::Html, routing::get, Json, Router};
use serde::Serialize;
use tokio::signal;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};



async fn index() -> Html<&'static str> {
    Html("<h1>Hello World!</h1>")
}

#[derive(Serialize)]
struct Test {
    text: String,
    salam: i32
}

async fn return_json() -> Json<Test> {
    Json(Test {
        text: String::from("salam"),
        salam: 1337
    })
}



#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();


    let port = 5000;
    let router = Router::new()
                    .route("/", get(index))
                    .route("/json", get(return_json))
                    .layer(TraceLayer::new_for_http());



    let listener = tokio::net::TcpListener::bind(format!("localhost:{port}"))
        .await
        .unwrap();

    println!("Listening on port {port}");

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
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
