use tracing::{info, Level};

fn main() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // builds the subscriber.
        .finish();

    tracing::subscriber::with_default(subscriber, || {
        info!("This will be logged to stdout");
        say_hello();
    });
    info!("This will _not_ be logged to stdout");
}

#[tracing::instrument(name = "hello")]
fn say_hello() {
    info!("Hello, world!");
}
