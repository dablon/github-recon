# GitHub Recon

CLI tool for GitHub repository reconnaissance with CSV, HTML, and Excel (XLSX) report generation. Includes MCP server for AI agent integration and auto-category detection.

## Features

- **Search GitHub** repositories with advanced query syntax
- **Generate CSV** reports with repository metadata
- **Generate HTML** reports with dark theme styling
- **Generate Excel (XLSX)** reports with multi-sheet, categorized output
- **Auto-category detection** from query keywords → creates category sheets
- **Configurable sorting** (stars, forks, updated)
- **MCP server** implementation for AI agent integration
- **14 E2E test scenarios** for automated validation
- **Docker deployment** ready (multi-stage build)

## Installation

### From Source

```bash
git clone https://github.com/dablon/github-recon.git
cd github-recon

# Option A: Build with Docker (recommended)
docker build -f Dockerfile.build -t github-recon-built .

# Option B: Build with Cargo (requires rust toolchain)
cargo build --release
./target/release/github-recon --help
```

### Using Docker (pre-built)

```bash
docker pull ghcr.io/dablon/github-recon:latest
# or build locally
docker build -t github-recon-built .
```

## Usage

### CLI Search - Basic

```bash
github-recon search "python" --limit 10
github-recon search "rust web framework" --limit 50
github-recon search "machine learning" --sort stars --order desc
```

### Excel (XLSX) Reports

Excel is the flagship format — generates professional multi-sheet reports with auto-categorization:

```bash
# XLSX output with --xlsx-output flag
github-recon search "pentest vulnerability" --limit 50 \
  --xlsx-output report.xlsx

# Auto-detect XLSX from -f xlsx flag
github-recon search "AI agent autonomous" --limit 100 -f xlsx \
  --output report.csv  # Generates report.xlsx

# Combined with CSV output
github-recon search "docker kubernetes" --limit 50 \
  -o data.csv -f both
```

### HTML Report

```bash
github-recon search "python" --html-output report.html
```

### CSV Only

```bash
github-recon search "python" --format csv -o results.csv
```

### Output Formats

| Format | Flag | Description |
|--------|------|-------------|
| CSV | `--format csv` | Standard CSV output |
| HTML | `--format html` or `--html-output` | Dark-themed HTML report |
| XLSX | `--format xlsx` or `--xlsx-output` | Excel with auto-categorization |
| Both | `--format both` | CSV + HTML + XLSX simultaneously |

### Sort Options

```bash
# Sort by stars (default)
github-recon search "rust" --sort stars --order desc

# Sort by forks
github-recon search "rust" --sort forks --order desc

# Sort by last updated
github-recon search "rust" --sort updated --order desc
```

## Excel Report Structure

When using `-f xlsx` or `--xlsx-output`, the generated Excel file contains:

1. **Results sheet** — All repositories sorted by stars (descending)
2. **Category sheet** — Auto-detected from query keywords (e.g., "pentest" → "Pentest")
3. **All Combined sheet** — Complete repository list for cross-referencing

### Auto-Category Detection

The CLI detects category from query keywords and creates dedicated sheets:

| Keyword | Category Sheet |
|---------|---------------|
| pentest, penetration | Pentest |
| vulnerability, cve | Vulnerability |
| bug bounty, bounty | Bug Bounty |
| recon, reconnaissance | Reconnaissance |
| network, port scan | Network Discovery |
| scanner | Scanner |
| mcp | MCP |
| ai agent, llm, autonomous | AI Agent |
| automation | Automation |
| exploit, payload | Exploit |
| security, cyber | Security |

Example:
```bash
github-recon search "AI pentest autonomous vulnerability" --limit 100 \
  -f xlsx --output security_report.xlsx
# Creates: Results | Pentest | Vulnerability | AI Agent | All Combined
```

### E2E Test Scenarios

Run automated tests to validate functionality:

```bash
# Build test image first
docker build -f Dockerfile.build -t github-recon-built .

# Run E2E tests (14 scenarios)
./tests/e2e.sh

# Individual test examples
docker run --rm -e GITHUB_TOKEN=$GH_TOKEN github-recon-built search "python" --limit 5
docker run --rm -e GITHUB_TOKEN=$GH_TOKEN github-recon-built search "rust" -f xlsx --xlsx-output /tmp/rust.xlsx
```

Test coverage:
- Help flags and CLI help
- CSV format output
- HTML format output
- XLSX format output with file verification
- Sort by stars/forks/updated
- Sort order (asc/desc)
- Limit parameter validation
- MCP mode
- Empty result handling
- Multi-format generation
- Category detection

## Full Command Options

```
github-recon search QUERY [OPTIONS]

Arguments:
  QUERY          Search query string (required)

Options:
  -o, --output PATH       Output CSV file path
  -H, --html-output PATH  Output HTML file path
  -x, --xlsx-output PATH  Output Excel (XLSX) file path
  -f, --format FORMAT    Output format: csv, html, xlsx, both (default: both)
  -l, --limit N          Maximum number of results (default: 100, max: 100)
  -s, --sort FIELD        Sort by: stars, forks, updated (default: stars)
  -d, --order ORDER        Sort order: asc, desc (default: desc)
  -h, --help               Show help

github-recon mcp [OPTIONS]
  --stdio                 Run as MCP server (stdio mode)
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

## Docker Usage Examples

```bash
# XLSX report (recommended)
docker run --rm \
  -e GITHUB_TOKEN=$GITHUB_TOKEN \
  -v $(pwd)/reports:/output \
  github-recon-built search "AI pentest vulnerability" \
  --limit 100 -f xlsx --output /output/security-report.xlsx

# HTML report
docker run --rm \
  -e GITHUB_TOKEN=$GITHUB_TOKEN \
  -v $(pwd)/reports:/app/reports \
  github-recon-built search "kubernetes" --html-output /app/reports/k8s.html

# CSV with auto-generated HTML
docker run --rm \
  -e GITHUB_TOKEN=$GITHUB_TOKEN \
  -v $(pwd)/reports:/app/reports \
  github-recon-built search "rust" --limit 50 -o /app/reports/rust.csv

# Any topic - categories auto-detected
docker run --rm \
  -e GITHUB_TOKEN=$GITHUB_TOKEN \
  -v $(pwd)/reports:/output \
  github-recon-built search "YOUR_TOPIC_HERE" \
  --limit 100 -f xlsx --output /output/report.xlsx

# Combined formats
docker run --rm \
  -e GITHUB_TOKEN=$GITHUB_TOKEN \
  github-recon-built search "machine learning" \
  --limit 50 -o /tmp/ml.csv -f both
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `GITHUB_TOKEN` | GitHub Personal Access Token (required for higher rate limits) |

**Rate Limits (unauthenticated):** 60 requests/hour
**Rate Limits (authenticated):** 5,000 requests/hour

Get a token at: https://github.com/settings/tokens

## Quick Reference

```bash
# Excel report (any topic)
github-recon search "react" -f xlsx --output react.xlsx

# CSV + HTML + XLSX all at once
github-recon search "python" -o py.csv -f both

# Top 20 by forks
github-recon search "javascript" -l 20 -s forks -d desc -f xlsx --output top20.xlsx

# Docker
docker run --rm -e GITHUB_TOKEN=$GITHUB_TOKEN \
  github-recon-built search "devops" --limit 50 \
  -f xlsx --output /app/report.xlsx

# MCP server
github-recon mcp --stdio
```

## MCP Server

The tool includes an MCP (Model Context Protocol) server for AI agent integration:

```bash
github-recon mcp --stdio
```

### Tools

#### tools/list
Returns available tools for AI agent integration.

#### tools/call

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

## Architecture

```
github-recon/
├── src/
│   ├── main.rs        # CLI entry point with clap
│   ├── api.rs         # GitHub API client
│   ├── csv.rs         # CSV generation
│   ├── html.rs        # HTML report generation
│   ├── xlsx.rs        # XLSX wrapper (calls Python)
│   ├── mcp.rs         # MCP server implementation
│   └── lib.rs         # Library exports
├── scripts/
│   └── gen_excel.py   # Python/openpyxl Excel generator
├── tests/
│   └── e2e.sh         # 14 E2E test scenarios
├── Dockerfile.build   # Multi-stage Docker build
└── README.md
```

**XLSX Generation:** Rust wrapper (`xlsx.rs`) writes CSV to temp file, calls embedded Python script (`gen_excel.py`) which uses `openpyxl` to generate the Excel workbook with formatting, freeze panes, and category sheets.

## License

Private - All rights reserved