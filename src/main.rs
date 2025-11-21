// moved modules into workspace crates

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize application
    let state = app::initialize_app().await?;

    // Run REPL loop
    app::run_repl(state).await?;

    Ok(())
}
