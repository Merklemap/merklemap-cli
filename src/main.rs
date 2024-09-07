//! MerkleMap CLI
//!
//! This module provides the command-line interface for interacting with the [merklemap](https://www.merklemap.com/) API.
//! It allows users to search for subdomains and tail live subdomain discoveries.

use clap::{Parser, Subcommand};
use merklemap_cli::{search, tail};

#[derive(Parser)]
#[command(author, version, about = "A merklemap.com api client", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search for subdomains matching a given query.
    Search { query: String },
    /// Tail live subdomains discoveries from the Merklemap ingestion pipeline.
    Tail,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Search { query } => {
            search(query).await?;
        }
        Commands::Tail => {
            tail().await?;
        }
    }

    Ok(())
}
