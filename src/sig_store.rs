use sequoia_openpgp::{packet::Signature, Cert};

pub trait SigStore {
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
