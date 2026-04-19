# GitHub Recon

CLI tool for GitHub repository reconnaissance with CSV output and MCP server for AI agent integration.

## Features

- Search GitHub repositories with advanced query syntax
- Generate CSV reports with repository metadata
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
  -o, --output PATH    Output CSV file path
  -l, --limit N        Maximum number of results (default: 100)
  -s, --sort FIELD     Sort by: stars, forks, updated (default: stars)
  -d, --order ORDER   Sort order: asc, desc (default: desc)
  -h, --help          Show help
```

## Output Format

CSV with columns:

| Column | Description |
|--------|-------------|
| url | Repository URL |
| name | Full repository name (owner/repo) |
| description | Repository description |
| stars | Star count |
| forks | Fork count |
| last_updated | Last updated timestamp |
| language | Primary programming language |

## Examples

### Example 1: DevOps Python Projects

```bash
$ github-recon search "topic:devops language:python" --limit 5

Searching GitHub for: topic:devops language:python
Limit: 5, Sort: stars (desc)

Found 5 repositories:
Wrote 5 repositories to CSV.
```

**Output CSV:**
```csv
url,name,description,stars,forks,last_updated,language
https://github.com/bregman-arie/devops-exercises,bregman-arie/devops-exercises,"Linux, Jenkins, AWS, SRE, Prometheus, Docker, Python, Ansible...",82079,19229,2026-04-19T17:48:12Z,Python
https://github.com/getsentry/sentry,getsentry/sentry,"Developer-first error tracking and performance monitoring",43615,4655,2026-04-19T17:30:13Z,Python
https://github.com/httpie/cli,httpie/cli,"HTTPie CLI - modern, user-friendly command-line HTTP client",37953,3910,2026-04-19T13:46:47Z,Python
```

### Example 2: Blockchain/Crypto Projects

```bash
$ github-recon search "stars:>1000 blockchain crypto" --limit 5

Searching GitHub for: stars:>1000 blockchain crypto
Limit: 5, Sort: stars (desc)

Found 5 repositories:
Wrote 5 repositories to CSV.
```

### Example 3: Save to File

```bash
$ github-recon search "machine learning python" --limit 20 --output ml-projects.csv

Searching GitHub for: machine learning python
Limit: 20, Sort: stars (desc)

Found 20 repositories:
Wrote 20 repositories to CSV.
```

### Example 4: Docker Usage

```bash
# With GitHub token for higher rate limits
docker run --rm \
  -e GITHUB_TOKEN=ghp_xxxxxxxxxxxx \
  -v $(pwd)/output:/app/output \
  github-recon search "docker" --limit 50 --output /app/output/docker.csv

# Without token (rate limited)
docker run --rm \
  -v $(pwd)/output:/app/output \
  github-recon search "kubernetes" --limit 30 --output /app/output/k8s.csv
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
# Simple search
github-recon search "react"

# With output file
github-recon search "python" -o python.csv

# Top 10 by stars
github-recon search "javascript" -l 10 -s stars -d desc

# Top 10 by forks
github-recon search "javascript" -l 10 -s forks -d desc

# Docker with token
docker run --rm -e GITHUB_TOKEN=$GITHUB_TOKEN github-recon search "rust" -l 100 -o rust.csv
```

## License

Private - All rights reserved
