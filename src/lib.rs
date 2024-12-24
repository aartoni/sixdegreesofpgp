#![feature(hash_set_entry)]
use sequoia_openpgp::{cert::CertParser, Cert};
use std::{path::PathBuf, rc::Rc};

mod graph;
mod sig_store;

pub use graph::Graph;
pub use sig_store::SigStore;

pub type Signee = Rc<String>;
pub type Signer = Rc<String>;

pub fn sync_cache() {
    println!("The sync feature hasn't been implemented yet");
}

pub fn get_cert_paths() -> impl Iterator<Item = PathBuf> {
    let mut path = dirs::cache_dir().expect("No cache dir found");
    path.push(env!("CARGO_PKG_NAME"));
    path.read_dir()
        .expect("Unable to read the cache directory")
        .map(|result| result.expect("msg"))
        .filter(|entry| entry.file_name().to_string_lossy().ends_with(".pgp"))
        .map(|entry| entry.path())
}

#[must_use]
pub fn get_certs(parser: CertParser) -> Vec<Cert> {
    parser.flatten().collect()
}
