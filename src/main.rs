use std::collections::HashSet;

use sequoia_openpgp::cert::prelude::*;
use sequoia_openpgp::parse::Parse;
use sixdegreesofpgp::{get_cert_paths, get_certs, sync_cache, WebOfTrustProvider};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    sync_cache();

    // Parse files
    let certs: Vec<_> = get_cert_paths()
        .map(CertParser::from_file)
        .flatten()
        .map(get_certs)
        .flatten()
        .collect();

    let nodes: HashSet<_> = certs
        .iter()
        .map(|c| c.fingerprint().to_spaced_hex())
        .collect();

    let edges: Vec<_> = certs.iter().flat_map(|c| c.get_edges()).collect();

    // TODO Write to DB
    println!("Nodes: {nodes:?}");
    println!("Edges: {edges:?}");

    // TODO Perform a simple query

    // TODO Display results
    Ok(())
}
