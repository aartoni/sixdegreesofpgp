use std::{env, fs};

use neo4rs::{ConfigBuilder, Graph, Result};

struct DatabaseUri(String);

impl Default for DatabaseUri {
    fn default() -> Self {
        Self("127.0.0.1:7687".to_string())
    }
}

struct DatabaseName(String);

impl Default for DatabaseName {
    fn default() -> Self {
        Self("neo4j".to_string())
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
    name: Option<DatabaseName>,
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
        let name = Some(DatabaseName(env::var("NEO4J_DB_NAME")?));
        let uri = Some(DatabaseUri(env::var("NEO4J_URI")?));
        let user = Some(DatabaseUser(env::var("NEO4J_USER")?));
        let pass = match env::var("NEO4J_PASS_FILE") {
            Ok(path) => fs::read_to_string(path)?.replace("neo4j/", ""),
            Err(_) => env::var("NEO4J_PASS")?,
        };
        let pass = Some(DatabasePassword(pass));
        Ok(Self {
            name,
            uri,
            user,
            pass,
        })
    }

    /// Builds a `Graph` instance using the provided settings.
    ///
    /// # Errors
    ///
    /// If creation fails, a `node4rs::Error` is returned.
    pub async fn build(self) -> Result<Graph> {
        let config = ConfigBuilder::new()
            .db(self.name.unwrap_or_default().0)
            .uri(self.uri.unwrap_or_default().0)
            .user(self.user.unwrap_or_default().0)
            .password(self.pass.unwrap_or_default().0)
            .build()?;
        Graph::connect(config).await
    }
}

impl Default for DatabaseBuilder {
    fn default() -> Self {
        Self {
            name: Some(DatabaseName::default()),
            uri: Some(DatabaseUri::default()),
            user: Some(DatabaseUser::default()),
            pass: Some(DatabasePassword::default()),
        }
    }
}
