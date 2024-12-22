use std::collections::HashSet;

use sequoia_openpgp::cert::prelude::*;
use sequoia_openpgp::packet::Signature;
use sequoia_openpgp::parse::Parse;
use sixdegreesofpgp::{get_cert_paths, get_certs, sync_cache};

type Signee = String;
type Signer = String;

trait SigStore {
    fn get_signatures(&self) -> impl Iterator<Item = &Signature>;
}

impl SigStore for Cert {
    /// Returns an iterator over third-party signatures (technically, certifications)
    fn get_signatures(&self) -> impl Iterator<Item = &Signature> {
        let user_id_signatures = self.userids().flat_map(|uid| uid.certifications());
        let subkey_signatures = self.keys().subkeys().flat_map(|sub| sub.certifications());
        // These are (almost) always empty
        let primary_key_signatures = self.primary_key().certifications();
        user_id_signatures
            .chain(subkey_signatures)
            .chain(primary_key_signatures)
    }
}

trait WebOfTrustProvider {
    fn get_edges(&self) -> impl Iterator<Item = (Signee, Signer)>;
}

impl WebOfTrustProvider for Cert {
    /// Gets the edges of the Web-of-Trust
    fn get_edges(&self) -> impl Iterator<Item = (Signee, Signer)> {
        // println!("{self:?}");
        self.get_signatures()
            .flat_map(|s| s.issuer_fingerprints())
            .map(|f| f.to_spaced_hex())
            // TODO Investigate signatures with no issuers, they're likely using KeyID
            // We did have support for KeyIDs in old versions
            .filter(|s| s.len() > 0)
            .map(|s| (self.fingerprint().to_spaced_hex(), s))
    }
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
