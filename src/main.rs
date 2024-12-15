use std::env;

use anyhow::Context;

use sequoia_openpgp::cert::prelude::*;
use sequoia_openpgp::parse::Parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO Sync keyserver cache

    // Read one file
    // TODO Read all the files (delay this operation to final stage)
    let mut path = dirs::cache_dir().expect("No cache dir found");
    path.push(env!("CARGO_PKG_NAME"));
    path.push("hkp-dump-0000.pgp");

    // Parse the file
    let parser = CertParser::from_file(path).context("Failed to create reader")?;

    let certs: Vec<_> = parser
        .into_iter()
        .filter(Result::is_ok)
        .map(|r| r.unwrap())
        .collect();

    // TODO Write to DB

    // TODO Perform a simple query

    // TODO Display results
    Ok(())
}
