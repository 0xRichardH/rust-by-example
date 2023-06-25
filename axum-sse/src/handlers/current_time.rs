use std::{convert::Infallible, time::Duration};

use async_stream::stream;
use axum::response::{
    sse::{Event, KeepAlive},
    Sse,
};
use chrono::Utc;
use futures_core::Stream;
use serde_json::json;

pub async fn current_time_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let s = stream! {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
                    let result = Event::default().json_data(json!({ "time": now }));
                    match result {
                        Ok(e) => yield Ok(e),
                        Err(err) => {
                            tracing::error!("Error: {:?}", err);
                            return;
                        }
                    }
                }
            }
        }
    };

    Sse::new(s).keep_alive(KeepAlive::default())
}
