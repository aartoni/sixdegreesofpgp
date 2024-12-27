#![feature(hash_set_entry)]
use dotenvy::dotenv;
use neo4rs::*;
use sequoia_openpgp::cert::prelude::*;
use sequoia_openpgp::parse::Parse;
use loader::{get_cert_paths, get_certs, get_db, sync_cache, Graph};
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
    tracing::info!("Database is ready.");
    sync_cache();

    // Parse files
    let certs = get_cert_paths()
        .map(CertParser::from_file)
        .flatten()
        .map(get_certs)
        .flatten();

    let mut graph = Graph::default();

    for cert in certs {
        graph.parse_cert(&cert);
    }

    tracing::info!("Finished parsing certificates.");

    // Write to DB
    let mut nodes_txn = db.start_txn().await.unwrap();
    let node_queries = graph
        .nodes()
        .iter()
        .map(|fp| query("MERGE (k:Key {fingerprint: $fp})").param("fp", fp.as_str()));
    nodes_txn.run_queries(node_queries).await.unwrap();
    nodes_txn.commit().await.unwrap();

    // Edges txn
    let mut edges_txn = db.start_txn().await.unwrap();
    let edge_queries = graph.edges().into_iter().map(|(signer, signee)| {
        query("MERGE (signer:Key {fingerprint: $signer}) MERGE (signee:Key {fingerprint: $signee}) MERGE (signer) -[:SIGNED]-> (signee)")
            .param("signer", signer.as_str())
            .param("signee", signee.as_str())
    });
    edges_txn.run_queries(edge_queries).await.unwrap();
    edges_txn.commit().await.unwrap();

    // Perform a simple query
    let mut results = db.execute(query("MATCH path=allShortestPaths((:Key {fingerprint:\"A6E68A783BDE4174672A4241F05CAA44E5518AFF\"})-[*]-(:Key {fingerprint:\"7A18807F100A4570C59684207E4E65C8720B706B\"})) RETURN path, length(path) as distance")).await.unwrap();

    // Display results
    while let Ok(Some(row)) = results.next().await {
        let path: Path = row.get("path").unwrap();
        tracing::info!("Path: {path:?}");
        let distance: u8 = row.get("distance").unwrap();
        tracing::info!("Distance: {distance}");
    }

    Ok(())
}
