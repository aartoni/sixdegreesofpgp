use cli::get_db;
use dotenvy::dotenv;
use neo4rs::*;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup
    dotenv()?;
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
    let db = get_db().await;

    // Perform a simple query
    let mut results = db.execute(query("MATCH path=allShortestPaths((:Key {fingerprint:\"A6E68A783BDE4174672A4241F05CAA44E5518AFF\"})-[*]-(:Key {fingerprint:\"7A18807F100A4570C59684207E4E65C8720B706B\"})) RETURN path, length(path) as distance")).await?;

    // Display results
    while let Ok(Some(row)) = results.next().await {
        let path: Path = row.get("path")?;
        tracing::info!("Path: {path:?}");
        let distance: u8 = row.get("distance")?;
        tracing::info!("Distance: {distance}");
    }

    Ok(())
}
