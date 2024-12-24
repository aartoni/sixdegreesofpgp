use std::{
    borrow::BorrowMut,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use sequoia_openpgp::{Cert, Fingerprint};

use crate::SigStore;

#[derive(Debug, Default)]
pub struct Graph {
    nodes: HashSet<Rc<String>>,
    raw_edges: HashMap<String, Rc<String>>,
    sub_keys: HashMap<String, Rc<String>>,
}

// Note for the future: signing another person's sub key makes no sense you
// always sign the primary key of a person. Unless it's a self-signature,
// which is not our case
impl Graph {
    fn parse_cert_signatures(&mut self, cert: &Cert, fp: Rc<String>) {
        cert.get_signatures()
            .flat_map(|s| s.issuer_fingerprints())
            // TODO Investigate signatures with no issuers, they're likely using KeyID
            // We did have support for KeyIDs in old versions
            .map(Fingerprint::to_hex)
            .for_each(|signer| {
                self.raw_edges.borrow_mut().insert(signer, fp.clone());
            });
    }

    fn parse_cert_subkeys(&mut self, cert: &Cert, fp: Rc<String>) {
        cert.keys()
            .subkeys()
            .map(|sk| sk.fingerprint().to_hex())
            .for_each(|sk| {
                self.sub_keys.borrow_mut().insert(sk, fp.clone());
            });
    }

    pub fn parse_cert(&mut self, cert: &Cert) {
        let fp = Rc::new(cert.fingerprint().to_hex());
        let fp = self.nodes.borrow_mut().get_or_insert(fp).clone();

        self.parse_cert_signatures(cert, fp.clone());
        self.parse_cert_subkeys(cert, fp.clone());
    }

    pub fn edges(&self) -> HashMap<Rc<String>, Rc<String>> {
        self.raw_edges
            .iter()
            .filter_map(|(signer, signee)| {
                self.sub_keys
                    .get(signer)
                    .map(|s| (s.clone(), signee.clone()))
            })
            .collect()
    }
}
