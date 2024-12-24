#![feature(hash_set_entry)]

use sequoia_openpgp::cert::prelude::*;
use sequoia_openpgp::parse::Parse;
use sixdegreesofpgp::{get_cert_paths, get_certs, sync_cache, Graph};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // TODO Write to DB
    // println!("Nodes: {nodes:?}");
    println!("Edges: {edges:?}");

    // TODO Perform a simple query

    // TODO Display results
    Ok(())
}
