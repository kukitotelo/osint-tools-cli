use color_eyre::Result;

mod action;
mod app;
mod components;
mod models;
mod tui;

use app::App;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize error handling
    color_eyre::install()?;

    // Initialize logging
    env_logger::init();

    // Create and run the application
    let mut app = App::new()?;
    app.run().await?;

    Ok(())
}
