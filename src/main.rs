mod api;
mod csv;
mod html;
mod mcp;
mod xlsx;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
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

        #[arg(short = 'o', long = "output", help = "Output CSV file path")]
        output: Option<PathBuf>,

        #[arg(short = 'H', long = "html-output", help = "Output HTML file path")]
        html_output: Option<PathBuf>,

        #[arg(short = 'x', long = "xlsx-output", help = "Output Excel (XLSX) file path")]
        xlsx_output: Option<PathBuf>,

        #[arg(
            short = 'f',
            long = "format",
            value_enum,
            default_value = "both",
            help = "Output format: csv, html, xlsx, or both"
        )]
        format: OutputFormat,

        #[arg(short = 'l', long = "limit", default_value = "100", help = "Maximum number of results")]
        limit: usize,

        #[arg(
            short = 's',
            long = "sort",
            value_enum,
            default_value = "stars",
            help = "Sort by: stars, forks, updated"
        )]
        sort: SortField,

        #[arg(
            short = 'd',
            long = "order",
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

#[derive(ValueEnum, Clone, Debug)]
enum OutputFormat {
    Csv,
    Html,
    Xlsx,
    Both,
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
    html_output: Option<PathBuf>,
    xlsx_output: Option<PathBuf>,
    format: OutputFormat,
    limit: usize,
    sort: SortField,
    order: SortOrder,
) -> Result<()> {
    let client = api::GitHubClient::new();

    // Auto-detect format from output flags
    let format = if xlsx_output.is_some() && output.is_none() && html_output.is_none() {
        OutputFormat::Xlsx
    } else if html_output.is_some() && output.is_none() && xlsx_output.is_none() {
        OutputFormat::Html
    } else {
        format
    };

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

    match format {
        OutputFormat::Csv | OutputFormat::Both => {
            if let Some(ref path) = output {
                let count = csv::write_csv_to_file(&repos, &path)?;
                println!("Wrote {} repositories to CSV: {}", count, path.display());
            } else {
                csv::write_csv_to_stdout(&repos)?;
            }
        }
        _ => {}
    }

    match format {
        OutputFormat::Html | OutputFormat::Both => {
            if let Some(ref path) = html_output {
                let count = html::write_html_to_file(&repos, &query, &path)?;
                println!("Wrote {} repositories to HTML: {}", count, path.display());
            } else if let Some(ref path) = output {
                let html_path = path.with_extension("html");
                let count = html::write_html_to_file(&repos, &query, &html_path)?;
                println!("Wrote {} repositories to HTML: {}", count, html_path.display());
            } else {
                let html = html::generate_html(&repos, &query);
                println!("{}", html);
            }
        }
        _ => {}
    }

    match format {
        OutputFormat::Xlsx | OutputFormat::Both => {
            if let Some(ref path) = xlsx_output {
                let count = xlsx::write_xlsx_to_file(&repos, &path, &query)?;
                println!("Wrote {} repositories to XLSX: {}", count, path.display());
            } else if let Some(ref path) = output {
                let xlsx_path = path.with_extension("xlsx");
                let count = xlsx::write_xlsx_to_file(&repos, &xlsx_path, &query)?;
                println!("Wrote {} repositories to XLSX: {}", count, xlsx_path.display());
            }
        }
        _ => {}
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
            html_output,
            xlsx_output,
            format,
            limit,
            sort,
            order,
        } => {
            run_search(query, output, html_output, xlsx_output, format, limit, sort, order).await?;
        }
        Commands::Mcp { .. } => {
            run_mcp()?;
        }
    }

    Ok(())
}