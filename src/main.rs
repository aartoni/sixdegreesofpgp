#![feature(hash_set_entry)]
use dotenvy::dotenv;
use neo4rs::*;
use sequoia_openpgp::cert::prelude::*;
use sequoia_openpgp::parse::Parse;
use sixdegreesofpgp::{get_cert_paths, get_certs, get_db, sync_cache, Graph};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup
    let db = get_db().await;
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_env_filter(EnvFilter::from_default_env())
        .compact()
        .init();

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
    let edges = graph.edges();

    // Write to DB

    // Add nodes
    let mut txn = db.start_txn().await.unwrap();
    let queries = graph
        .nodes()
        .iter()
        .map(|fp| query("CREATE (k:Key {fingerprint: $fp})").param("fp", fp.as_str()));
    txn.run_queries(queries).await.unwrap();
    txn.commit().await.unwrap();

    // TODO Perform a simple query

    // TODO Display results
    Ok(())
}
