#![feature(hash_set_entry)]
use neo4rs::*;
use sequoia_openpgp::cert::prelude::*;
use sequoia_openpgp::parse::Parse;
use sixdegreesofpgp::{get_cert_paths, get_certs, get_db, sync_cache, Graph};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup
    let db = get_db().await;
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
