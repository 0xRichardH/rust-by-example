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
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_handler))
        .route("/time", get(current_time_handler));

    println!("Listening on 0.0.0.0:8080");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
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
