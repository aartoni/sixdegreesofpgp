use clap::{Arg, Command};
use dotenvy::dotenv;
use libsdop::db::DatabaseBuilder;
use neo4rs::{query, Path};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup
    dotenv()?;
    let db = DatabaseBuilder::from_env()?.build().await?;

    // Parse args
    let matches = Command::new("cli")
        .arg(Arg::new("from").required(true))
        .arg(Arg::new("to").required(true))
        .get_matches();

    let from: &str = matches.get_one::<String>("from").unwrap();
    let to: &str = matches.get_one::<String>("to").unwrap();

    // Perform a simple query
    let mut results = db.execute(query("MATCH path=allShortestPaths((:Key {fingerprint: $from})-[*]-(:Key {fingerprint: $to})) RETURN path, length(path) as distance").param("from", from).param("to", to)).await?;

    // Display results
    while let Ok(Some(row)) = results.next().await {
        let distance: u8 = row.get("distance")?;
        println!("Distance: {distance}");
        let path: Path = row.get("path")?;
        let nodes = path
            .nodes()
            .into_iter()
            .flat_map(|n| n.get::<String>("fingerprint"));
        println!("{from}");
        nodes
            .skip(1)
            .take(distance as usize - 1)
            .for_each(|node| println!("...{node}"));
        println!("{to}");
        break;
    }

    Ok(())
}
