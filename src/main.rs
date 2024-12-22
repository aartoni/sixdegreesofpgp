use std::env;

use anyhow::Context;

use sequoia_openpgp::cert::prelude::*;
use sequoia_openpgp::parse::Parse;
use sequoia_openpgp::KeyHandle;

fn sync_cache() {
    println!("The sync feature hasn't been implemented yet");
}

fn get_cert_paths() -> impl Iterator<Item = PathBuf> {
    let mut path = dirs::cache_dir().expect("No cache dir found");
    path.push(env!("CARGO_PKG_NAME"));
    path.read_dir()
        .expect("Unable to read the cache directory")
        .map(|result| result.expect("msg"))
        .filter(|entry| entry.file_name().to_string_lossy().ends_with(".pgp"))
        // TODO Remove this limit
        .take(5)
        .map(|entry| entry.path())
}

fn get_certs(parser: CertParser) -> Vec<Cert> {
    parser.flatten().collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    sync_cache();

    // Parse files
    let certs: Vec<_> = get_cert_paths()
        .map(CertParser::from_file)
        .flatten()
        .map(get_certs)
        .flatten()
        .collect();

    let nodes: Vec<_> = certs
        .iter()
        .map(|c| c.fingerprint().to_spaced_hex())
        .collect();

    let signatures = certs.iter().flat_map(|c| {
        c.primary_key()
            .certifications()
            .map(|s| (c.fingerprint(), s))
    });

    let mut edges = Vec::new();

    for (fingerprint, signature) in signatures {
        println!("Signature found!");
        let signee = fingerprint.to_spaced_hex();
        signature
            .get_issuers()
            .iter()
            .map(|i| match i {
                KeyHandle::Fingerprint(fp) => fp.to_spaced_hex(),
                KeyHandle::KeyID(id) => {
                    eprintln!("Signed using a KeyID! {id}");
                    id.to_spaced_hex()
                }
            })
            .for_each(|i| edges.push((signee.clone(), i)));
    }

    // TODO Write to DB
    println!("Nodes: {nodes:?}");
    println!("Edges: {edges:?}");

    // TODO Perform a simple query

    // TODO Display results
    Ok(())
}
