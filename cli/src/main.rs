use clap::{Arg, Command};
use dotenvy::dotenv;
use libsdop::db::DatabaseBuilder;
use neo4rs::{query, Path};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup
    dotenv()?;
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let db = DatabaseBuilder::from_env()?.build().await?;

    // Parse args
    let matches = Command::new("cli")
        .arg(Arg::new("from").required(true))
        .arg(Arg::new("to").required(true))
        .get_matches();

    let from: &str = matches.get_one::<String>("from").unwrap();
    let to: &str = matches.get_one::<String>("to").unwrap();
    tracing::debug!("Obtaining path from {from} to {to}");

    // Perform a simple query
    let mut results = db.execute(query("MATCH path=allShortestPaths((:Key {fingerprint: $from})-[*]-(:Key {fingerprint: $to})) RETURN path, length(path) as distance").param("from", from).param("to", to)).await?;

    // Display results
    while let Ok(Some(row)) = results.next().await {
        let path: Path = row.get("path")?;
        tracing::trace!("Path: {path:?}");
        let nodes: Vec<_> = path
            .nodes()
            .into_iter()
            .flat_map(|n| n.get::<String>("fingerprint"))
            .collect();
        tracing::info!("Nodes: {nodes:?}");
        let distance: u8 = row.get("distance")?;
        tracing::info!("Distance: {distance}");
    }

    Ok(())
}
