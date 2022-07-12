mod config;
use eyre::Result;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = config::AppConfig::from_json5("config.json5")?;
    println!("Hello, world!");
    Ok(())
}
