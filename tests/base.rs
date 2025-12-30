use safeline_rs::Client;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

pub fn get_client() -> Client {
    init_log();
    Client::new(
        "https://127.0.0.1:9443/api",
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    )
}

pub fn init_log() {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::DEBUG)
        // completes the builder.
        .finish();

    // Only set if not already set
    let _ = tracing::subscriber::set_global_default(subscriber);
}
