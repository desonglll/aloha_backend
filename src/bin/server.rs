use aloha_backend::configuration::get_configuration;
use aloha_backend::startup::Application;
use tracing::Level;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("actix_web=debug,actix_server::worker=debug")
        .with_max_level(Level::DEBUG)
        .pretty() // 以更加人类可读的格式输出
        .init();
    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
