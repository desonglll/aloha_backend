use aloha_backend::configuration::get_configuration;
use aloha_backend::startup::Application;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.\nPlease check the configuration files to ensure the sequence of table and top-level elements is correct.");

    let level_filter = match configuration.log_level.as_str() {
        "trace" => LevelFilter::TRACE,
        "debug" => LevelFilter::DEBUG,
        "info" => LevelFilter::INFO,
        "warn" => LevelFilter::WARN,
        "error" => LevelFilter::ERROR,
        _ => LevelFilter::INFO,
    };

    let fmt_layer = tracing_subscriber::fmt::layer().with_filter(level_filter);
    tracing_subscriber::registry().with(fmt_layer).init();

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
