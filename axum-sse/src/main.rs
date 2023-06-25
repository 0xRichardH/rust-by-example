use std::{convert::Infallible, time::Duration};

use axum::{
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
    routing::get,
    Router,
};
pub(crate) use chrono::Utc;
use futures_util::stream::{self, Stream};
use serde_json::json;
use tokio::signal;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_handler))
        .route("/time", get(current_time_handler));

    println!("Listening on 0.0.0.0:8080");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}

async fn hello_handler() -> &'static str {
    "Hello World"
}

async fn current_time_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| {
        let time = Utc::now().format("%y-%m-%d %H:%M:%S").to_string();
        let result = Event::default().json_data(json!({ "time": time }));
        match result {
            Ok(event) => event,
            Err(_) => Event::default().data("error"),
        }
    })
    .map(Ok)
    .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(KeepAlive::default())
}
