use sequoia_openpgp::{cert::CertParser, Cert};
use std::path::PathBuf;

mod sig_store;

pub use sig_store::SigStore;

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
        // TODO Remove this limit
        .take(5)
        .map(|entry| entry.path())
}

pub fn get_certs(parser: CertParser) -> Vec<Cert> {
    parser.flatten().collect()
}
