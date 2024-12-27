#![feature(hash_set_entry)]
use neo4rs::{query, Result};
use sequoia_openpgp::{cert::CertParser, Cert};
use std::{path::PathBuf, rc::Rc};

mod graph;
mod sig_store;

pub use graph::Graph;
pub use sig_store::SigStore;

pub type Signee = Rc<String>;
pub type Signer = Rc<String>;

pub async fn setup_database(db: &neo4rs::Graph) {
    tracing::debug!("Setting up the database.");
    db.run(query("MATCH (n) DETACH DELETE n")).await.unwrap();
    tracing::debug!("....purged");
    db.run(query(
        "CREATE CONSTRAINT unique_fp IF NOT EXISTS FOR (k:Key) REQUIRE k.fingerprint IS UNIQUE",
    ))
    .await
    .unwrap();
    tracing::debug!("....constraint set");
}

pub fn sync_cache() {
    tracing::warn!("The sync feature hasn't been implemented yet");
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

pub async fn get_db() -> Result<neo4rs::Graph> {
    let uri = "127.0.0.1:7687";
    let user = "neo4j";
    let pass = "justice-welcome-sphere-jazz-anagram-6191";
    let db = neo4rs::Graph::new(uri, user, pass).await?;
    setup_database(&db).await;
    Ok(db)
}
