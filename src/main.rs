use config::{Config, File};
use std::env;
use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
  let settings = Config::builder()
  .add_source(File::with_name("config.toml"))
  .build()?;

  // Access the configuration values
  let database_url: String = settings.get("database.url").unwrap();

  let connection = sea_orm::Database::connect(&database_url).await?;
  Migrator::up(&connection, None).await?;

  api::rocket().await;

  return Ok(());
}