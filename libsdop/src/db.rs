use std::{env, fs};

use neo4rs::{Graph, Result};

struct DatabaseUri(String);

impl Default for DatabaseUri {
    fn default() -> Self {
        Self("127.0.0.1:7687".to_string())
    }
}

struct DatabaseUser(String);

impl Default for DatabaseUser {
    fn default() -> Self {
        Self("neo4j".to_string())
    }
}

struct DatabasePassword(String);

impl Default for DatabasePassword {
    fn default() -> Self {
        Self("justice-welcome-sphere-jazz-anagram-6191".to_string())
    }
}

pub struct DatabaseBuilder {
    uri: Option<DatabaseUri>,
    user: Option<DatabaseUser>,
    pass: Option<DatabasePassword>,
}

impl DatabaseBuilder {
    /// Provides a configured `DatabaseBuilder` instance by reading the environment.
    ///
    /// # Errors
    ///
    /// Returns an `env::VarError` in case the variable is unreadable and `io::Error` if the file doesn't exist.
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let uri = Some(DatabaseUri(env::var("NEO4J_URI")?));
        let user = Some(DatabaseUser(env::var("NEO4J_USER")?));
        let pass = match env::var("NEO4J_PASS_FILE") {
            Ok(path) => fs::read_to_string(path)?,
            Err(_) => env::var("NEO4J_PASS")?,
        };
        let pass = Some(DatabasePassword(pass));
        Ok(Self { uri, user, pass })
    }

    /// Builds a `Graph` instance using the provided settings.
    ///
    /// # Errors
    ///
    /// If creation fails, a `node4rs::Error` is returned.
    pub async fn build(self) -> Result<Graph> {
        Graph::new(
            self.uri.unwrap_or_default().0,
            self.user.unwrap_or_default().0,
            self.pass.unwrap_or_default().0,
        )
        .await
    }
}

impl Default for DatabaseBuilder {
    fn default() -> Self {
        Self {
            uri: Some(DatabaseUri::default()),
            user: Some(DatabaseUser::default()),
            pass: Some(DatabasePassword::default()),
        }
    }
}
