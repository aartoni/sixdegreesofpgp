use std::env;

use anyhow::Context;

use sequoia_openpgp::cert::prelude::*;
use sequoia_openpgp::parse::Parse;
use sequoia_openpgp::KeyHandle;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO Sync keyserver cache

    // Read one file
    // TODO Read all the files (delay this operation to final stage)
    let mut path = dirs::cache_dir().expect("No cache dir found");
    path.push(env!("CARGO_PKG_NAME"));
    path.push("hkp-dump-0000.pgp");

    // Parse the file
    let parser = CertParser::from_file(path).context("Failed to create reader")?;

    let certs: Vec<_> = parser.flatten().collect();
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
    drop(nodes);
    drop(edges);

    // TODO Perform a simple query

    // TODO Display results
    Ok(())
}
