mod api;
mod csv;
mod mcp;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "github-recon",
    about = "GitHub reconnaissance tool for searching repositories",
    version = "0.1.0",
    author = "github-recon"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Search {
        #[arg(help = "Search query string", required = true)]
        query: String,

        #[arg(short, long, help = "Output CSV file path")]
        output: Option<PathBuf>,

        #[arg(short, long, default_value = "100", help = "Maximum number of results")]
        limit: usize,

        #[arg(
            short,
            long,
            value_enum,
            default_value = "stars",
            help = "Sort by: stars, forks, updated"
        )]
        sort: SortField,

        #[arg(
            short,
            long,
            value_enum,
            default_value = "desc",
            help = "Sort order: asc, desc"
        )]
        order: SortOrder,
    },
    Mcp {
        #[arg(
            long,
            help = "Run as MCP server instead of CLI"
        )]
        _stdio: bool,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum SortField {
    Stars,
    Forks,
    Updated,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum SortOrder {
    Asc,
    Desc,
}

impl SortField {
    fn as_str(&self) -> &str {
        match self {
            SortField::Stars => "stars",
            SortField::Forks => "forks",
            SortField::Updated => "updated",
        }
    }
}

impl SortOrder {
    fn as_str(&self) -> &str {
        match self {
            SortOrder::Asc => "asc",
            SortOrder::Desc => "desc",
        }
    }
}

async fn run_search(
    query: String,
    output: Option<PathBuf>,
    limit: usize,
    sort: SortField,
    order: SortOrder,
) -> Result<()> {
    let client = api::GitHubClient::new();

    let search_query = api::SearchQuery::new(&query)
        .with_pagination(1, limit.min(100))
        .with_sort(sort.as_str(), order.as_str());

    println!("Searching GitHub for: {}", query);
    println!("Limit: {}, Sort: {} ({})", limit, sort.as_str(), order.as_str());
    println!();

    let repos = client.search_all_repositories(&search_query, Some(limit)).await?;

    if repos.is_empty() {
        println!("No repositories found.");
        return Ok(());
    }

    println!("Found {} repositories:", repos.len());

    let count = if let Some(ref path) = output {
        csv::write_csv_to_file(&repos, &path)?
    } else {
        csv::write_csv_to_stdout(&repos)?
    };

    if output.is_some() {
        println!("\nWrote {} repositories to CSV.", count);
    }

    Ok(())
}

fn run_mcp() -> Result<()> {
    mcp::run_mcp_server()
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Search {
            query,
            output,
            limit,
            sort,
            order,
        } => {
            run_search(query, output, limit, sort, order).await?;
        }
        Commands::Mcp { .. } => {
            run_mcp()?;
        }
    }

    Ok(())
}
