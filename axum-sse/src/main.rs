mod handlers;

use crate::handlers::{current_time, hello};

use std::sync::Mutex;
use std::time::Duration;
use std::{net::SocketAddr, sync::Arc};

use axum::routing::post;
use axum::{
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::Response,
    routing::get,
    Router,
};

use tokio::signal;
use tokio::sync::oneshot;
use tower_http::{classify::ServerErrorsFailureClass, trace::TraceLayer};
use tracing::{info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Default, Clone)]
pub struct AppState {
    stream_handlers: Arc<Mutex<Vec<oneshot::Sender<()>>>>,
}

#[tokio::main]
async fn main() {
    init_tracing_subscriber();

    let shared_state = AppState::default();
    let stream_handlers = Arc::clone(&shared_state.stream_handlers);
    let app = Router::new()
        .route("/", get(hello::hello_handler))
        .route("/time", get(current_time::current_time_handler))
        .route(
            "/terminate",
            post(current_time::terminate_connections_handler),
        )
        .with_state(shared_state)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(|request: &Request<_>, span: &Span| {
                    // You can use `_span.record("some_other_field", value)` in one of these
                    // closures to attach a value to the initially empty field in the info_span
                    // created above.
                    span.record("request path:", request.uri().path());
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
                    // ...
                })
                .on_eos(
                    |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                        // ...
                    },
                )
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        // ...
                    },
                ),
        );

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal(stream_handlers))
        .await
        .unwrap();
}

fn init_tracing_subscriber() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "axum_sse=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();
}

async fn shutdown_signal(stream_handlers: Arc<Mutex<Vec<oneshot::Sender<()>>>>) {
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

    let clear_stream_handlers = || {
        let handlers = std::mem::take(&mut *stream_handlers.lock().unwrap());
        for tx in handlers {
            _ = tx.send(());
        }
    };

    tokio::select! {
        _ = ctrl_c => {
            clear_stream_handlers();
        },
        _ = terminate => {
            clear_stream_handlers();
        },
    }

    tracing::info!("signal received, starting graceful shutdown");
}
