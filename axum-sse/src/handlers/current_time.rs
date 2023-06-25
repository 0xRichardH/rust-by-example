use std::{convert::Infallible, time::Duration};

use axum::response::{
    sse::{Event, KeepAlive},
    Sse,
};
use chrono::Utc;
use futures_util::{stream, Stream};
use serde_json::json;
use tokio_stream::StreamExt;

pub async fn current_time_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| {
        let time = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        tracing::info!("Time: {}", time);
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
