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

    // Perform a simple query
    let mut results = db.execute(query("MATCH path=allShortestPaths((:Key {fingerprint:\"A6E68A783BDE4174672A4241F05CAA44E5518AFF\"})-[*]-(:Key {fingerprint:\"7A18807F100A4570C59684207E4E65C8720B706B\"})) RETURN path, length(path) as distance")).await?;

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
