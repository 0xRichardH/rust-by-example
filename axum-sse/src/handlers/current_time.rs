use std::{convert::Infallible, time::Duration};

use async_stream::stream;
use axum::{
    extract::State,
    response::{
        sse::{Event, KeepAlive},
        Sse,
    },
};
use chrono::Utc;
use futures_core::Stream;
use serde_json::json;
use tokio::sync::oneshot;

use crate::AppState;

pub async fn current_time_handler(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let (tx, mut rx) = oneshot::channel::<()>();

    state.stream_handlers.lock().unwrap().push(tx);
    tracing::info!("Adding new connection");

    let s = stream! {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
                    let result = Event::default().json_data(json!({ "time": now }));
                    match result {
                        Ok(e) => {
                            tracing::info!("Sending event: {:?}", e);
                            yield Ok(e)
                        },
                        Err(err) => {
                            tracing::error!("Error: {:?}", err);
                            return;
                        }
                    }
                }
                _ =&mut rx => {
                    tracing::info!("Terminate connections");
                    return;
                }
            }
        }
    };

    Sse::new(s).keep_alive(KeepAlive::default())
}

pub async fn terminate_connections_handler(State(state): State<AppState>) -> &'static str {
    let stream_handlers = std::mem::take(&mut *state.stream_handlers.lock().unwrap());
    for tx in stream_handlers {
        _ = tx.send(());
    }

    "Done"
}
