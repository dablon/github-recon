# GitHub Recon

CLI tool for GitHub repository reconnaissance with CSV and HTML report generation. Includes MCP server for AI agent integration.

## Features

- Search GitHub repositories with advanced query syntax
- Generate CSV reports with repository metadata
- Generate professional HTML reports with dark theme
- Configurable sorting (stars, forks, updated)
- MCP server implementation for AI agent integration
- Docker deployment ready

## Installation

### From Source

```bash
git clone https://github.com/dablon/github-recon.git
cd github-recon
cargo build --release
./target/release/github-recon --help
```

### Using Docker

```bash
docker build -t github-recon .
docker run --rm -e GITHUB_TOKEN github-recon search "python" --limit 10
```

## Usage

### CLI Search

```bash
github-recon search "topic:devops language:python" --limit 10 --output results.csv
```

### HTML Report

```bash
github-recon search "python" --html-output report.html
```

### Output to stdout

```bash
github-recon search "stars:>1000 machine learning" --limit 20
```

### Sort Options

```bash
# Sort by stars (default)
github-recon search "rust" --sort stars --order desc

# Sort by forks
github-recon search "rust" --sort forks --order desc

# Sort by last updated
github-recon search "rust" --sort updated --order desc
```

### Full Command Options

```
github-recon search QUERY [OPTIONS]

Arguments:
  QUERY    Search query string

Options:
  -o, --output PATH      Output CSV file path
  -H, --html-output PATH Output HTML file path
  -f, --format FORMAT    Output format: csv, html, or both (default: both if output specified)
  -l, --limit N         Maximum number of results (default: 100)
  -s, --sort FIELD      Sort by: stars, forks, updated (default: stars)
  -d, --order ORDER     Sort order: asc, desc (default: desc)
  -h, --help            Show help
```

## Output Formats

### CSV

| Column | Description |
|--------|-------------|
| url | Repository URL |
| name | Full repository name (owner/repo) |
| description | Repository description |
| stars | Star count |
| forks | Fork count |
| last_updated | Last updated timestamp |
| language | Primary programming language |

### HTML Report

Professional dark-themed report with:
- GitHub-inspired styling
- Statistics cards (total stars, forks, languages)
- Language color coding
- Responsive design for mobile
- Links to repository pages

**Example HTML output:**

```
┌─────────────────────────────────────────────────────────────┐
│  python                                           [HEADER] │
│  📊 5 repositories  🔍 GitHub Search  📅 Generated 2026... │
├─────────────────────────────────────────────────────────────┤
│  Total Stars │ Total Forks │ Languages │ Repositories       │
│  1.3M        │ 223K        │ 3         │ 5                  │
├─────────────────────────────────────────────────────────────┤
│  # │ Repository          │ Description    │ Stars │ Forks  │
│  1 │ donnemartin/        │ Learn how to   │ 343K  │ 55.4K  │
│    │ system-design-primer │ design systems │       │        │
│  2 │ vinta/awesome-python│ Python libs    │ 293K  │ 27.7K  │
│  3 │ practical-tutorials/│ Project-based  │ 263K  │ 34.2K  │
│    │ project-based-learning│ tutorials     │       │        │
└─────────────────────────────────────────────────────────────┘
```

## Examples

### Example 1: HTML Report

```bash
$ github-recon search "python" --html-output python-report.html

Searching GitHub for: python
Limit: 100, Sort: stars (desc)

Found 100 repositories:
Wrote 100 repositories to HTML: python-report.html
```

### Example 2: Both CSV and HTML

```bash
$ github-recon search "machine learning" -o ml-report.csv --format both

Searching GitHub for: machine learning
Limit: 100, Sort: stars (desc)

Found 100 repositories:
Wrote 100 repositories to CSV: ml-report.csv
Wrote 100 repositories to HTML: ml-report.html
```

### Example 3: CSV Only

```bash
$ github-recon search "devops" --format csv -o devops.csv

Searching GitHub for: devops
Limit: 100, Sort: stars (desc)

Found 100 repositories:
Wrote 100 repositories to CSV: devops.csv
```

### Example 4: Docker Usage

```bash
# HTML report
docker run --rm \
  -e GITHUB_TOKEN=$GITHUB_TOKEN \
  -v $(pwd)/reports:/app/reports \
  github-recon search "kubernetes" --html-output /app/reports/k8s.html

# Both formats
docker run --rm \
  -e GITHUB_TOKEN=$GITHUB_TOKEN \
  -v $(pwd)/reports:/app/reports \
  github-recon search "rust" --limit 50 -o /app/reports/rust.csv --format both
```

### Example 5: Quick Search with Auto-HTML

If you specify `--output` without `--format`, the HTML is auto-generated with the same filename but .html extension:

```bash
$ github-recon search "docker" -o docker.csv
# Generates both docker.csv and docker.html
```

## GitHub Search Syntax

Use GitHub's advanced search query syntax:

| Query | Description |
|-------|-------------|
| `language:python` | Filter by language |
| `stars:>1000` | Minimum stars |
| `forks:>100` | Minimum forks |
| `topic:devops` | Filter by topic |
| `user:octocat` | Specific owner |
| `pushed:>2024-01-01` | Recently updated |
| `created:>2020-01-01` | Created after date |

**Combine queries:**
```bash
github-recon search "stars:>5000 language:rust topic:webassembly"
github-recon search "user:dablon forks:>10"
github-recon search "stars:>100 stars:<1000 python"
```

## MCP Server

The tool includes an MCP (Model Context Protocol) server for AI agent integration:

```bash
github-recon mcp
```

The MCP server accepts JSON-RPC requests with:

### tools/list

Returns available tools:
```json
{"jsonrpc":"2.0","id":1,"method":"tools/list"}
```

### tools/call

Search repositories:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "search_repositories",
    "arguments": {
      "query": "python machine learning",
      "limit": 100,
      "sort": "stars",
      "order": "desc"
    }
  }
}
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `GITHUB_TOKEN` | GitHub Personal Access Token (for higher rate limits) |

**Rate Limits (unauthenticated):** 60 requests/hour
**Rate Limits (authenticated):** 5,000 requests/hour

## Quick Reference

```bash
# HTML report (default 100 repos)
github-recon search "react" --html-output report.html

# CSV output
github-recon search "python" -o python.csv

# Both formats
github-recon search "rust" -o rust.csv --format both

# Top 20 by forks
github-recon search "javascript" -l 20 -s forks -d desc --html-output top20.html

# Docker
docker run --rm -e GITHUB_TOKEN=$GITHUB_TOKEN github-recon search "devops" --html-output /app/report.html
```

## License

Private - All rights reserved
