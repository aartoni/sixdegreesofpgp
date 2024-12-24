#![feature(hash_set_entry)]
use std::collections::{HashMap, HashSet};

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

    // This can be removed afterwords since we upsert values while mapping subkeys
    let mut nodes: HashSet<_> = certs.iter().map(|c| c.fingerprint().to_hex()).collect();

    let subkeys_map: HashMap<_, _> = certs
        .iter()
        .map(|c| (c, c.fingerprint().to_hex()))
        .map(|(c, fp)| (c.keys().subkeys(), nodes.get_or_insert(fp).clone()))
        .flat_map(|(keys, fp)| keys.map(move |k| (k, fp.clone())))
        .map(|(k, fp)| (k.fingerprint().to_hex(), fp))
        .collect();

    let edges: Vec<_> = certs.iter().flat_map(|c| c.get_edges()).collect();

    // TODO Write to DB
    println!("Nodes: {nodes:?}");
    println!("Edges: {edges:?}");

    // TODO Perform a simple query

    // TODO Display results
    Ok(())
}
