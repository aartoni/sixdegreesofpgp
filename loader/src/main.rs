#![feature(hash_set_entry)]
use dotenvy::dotenv;
use itertools::Itertools;
use libsdop::db::DatabaseBuilder;
use loader::{get_cert_paths, get_certs, setup_database, sync_cache, Graph};
use neo4rs::query;
use sequoia_openpgp::cert::prelude::*;
use sequoia_openpgp::parse::Parse;
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
    setup_database(&db).await?;
    tracing::info!("Database is ready.");
    sync_cache();

    // Parse files
    let certs = get_cert_paths()
        .flat_map(CertParser::from_file)
        .flat_map(get_certs);

    let mut graph = Graph::default();

    for cert in certs {
        graph.parse_cert(&cert);
    }

    tracing::info!("Finished parsing certificates.");

    // Write to DB
    let node_queries = graph
        .nodes()
        .iter()
        .map(|fp| query("MERGE (k:Key {fingerprint: $fp})").param("fp", fp.as_str()))
        .chunks(100);

    for query in &node_queries {
        let mut nodes_txn = db.start_txn().await?;
        nodes_txn.run_queries(query).await?;
        nodes_txn.commit().await?;
    }

    // Edges txn
    let edge_queries = graph.edges().into_iter().map(|(signer, signee)| {
        query("MERGE (signer:Key {fingerprint: $signer}) MERGE (signee:Key {fingerprint: $signee}) MERGE (signer) -[:SIGNED]-> (signee)")
        .param("signer", signer.as_str())
        .param("signee", signee.as_str())
    }).chunks(100);

    for query in &edge_queries {
        let mut edges_txn = db.start_txn().await?;
        edges_txn.run_queries(query).await?;
        edges_txn.commit().await?;
    }

    tracing::info!("Done loading nodes and edges");

    Ok(())
}
