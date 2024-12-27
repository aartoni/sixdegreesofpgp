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
