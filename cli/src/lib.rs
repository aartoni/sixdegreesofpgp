use neo4rs::Result;

pub async fn get_db() -> Result<neo4rs::Graph> {
    let uri = "127.0.0.1:7687";
    let user = "neo4j";
    let pass = "justice-welcome-sphere-jazz-anagram-6191";
    neo4rs::Graph::new(uri, user, pass).await
}
