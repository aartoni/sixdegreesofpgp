use std::{collections::HashMap, rc::Rc};

use crate::{SigStore, Signee, Signer};
use sequoia_openpgp::{Cert, Fingerprint};

pub trait WebOfTrustProvider {
    fn get_edges(
        &self,
        subkeys_map: &HashMap<String, Rc<String>>,
    ) -> impl Iterator<Item = (Signee, Signer)>;
}

impl WebOfTrustProvider for Cert {
    /// Gets the edges of the Web-of-Trust
    fn get_edges(
        &self,
        subkeys_map: &HashMap<String, Rc<String>>,
    ) -> impl Iterator<Item = (Signee, Signer)> {
        self.get_signatures()
            .flat_map(|s| s.issuer_fingerprints())
            .map(Fingerprint::to_hex)
            // TODO Investigate signatures with no issuers, they're likely using KeyID
            // We did have support for KeyIDs in old versions
            .filter(|s| !s.is_empty())
            .map(|s| {
                let k = self.fingerprint().to_hex();
                (
                    subkeys_map.get(&k).unwrap_or(&Rc::new(k)).clone(),
                    subkeys_map.get(&s).unwrap_or(&Rc::new(s)).clone(),
                )
            })
    }
}
