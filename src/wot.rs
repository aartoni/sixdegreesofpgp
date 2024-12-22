use crate::{SigStore, Signee, Signer};
use sequoia_openpgp::Cert;

pub trait WebOfTrustProvider {
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
