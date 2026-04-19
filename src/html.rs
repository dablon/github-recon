use anyhow::Result;
use std::path::Path;

use crate::api::GitHubRepository;

pub fn write_html_to_file(repos: &[GitHubRepository], title: &str, path: &Path) -> Result<usize> {
    let html = generate_html(repos, title);
    std::fs::write(path, html)?;
    Ok(repos.len())
}

pub fn generate_html(repos: &[GitHubRepository], title: &str) -> String {
    let mut rows = String::new();
    
    for (i, repo) in repos.iter().enumerate() {
        let rank = i + 1;
        let stars = format_stars(repo.stargazers_count);
        let forks = format_number(repo.forks_count);
        let description = repo.description
            .as_ref()
            .map(|d| escape_html(d))
            .unwrap_or_default();
        let language = repo.language
            .as_ref()
            .map(|l| format!("<span class=\"lang lang-{}\">{}</span>", l.to_lowercase(), l))
            .unwrap_or_default();
        let updated = format_date(&repo.updated_at);
        
        rows.push_str(&format!(
            r##"
        <tr>
            <td class="rank">{}</td>
            <td class="name">
                <a href="{}" target="_blank">{}</a>
                {}
            </td>
            <td class="description">{}</td>
            <td class="stars">{}</td>
            <td class="forks">{}</td>
            <td class="updated">{}</td>
        </tr>
        "##,
            rank,
            repo.html_url,
            repo.full_name,
            language,
            description,
            stars,
            forks,
            updated
        ));
    }
    
    let total_stars = total_stars(repos);
    let total_forks = total_forks(repos);
    let unique_langs = unique_languages(repos);
    
    format!(r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{} - GitHub Recon Report</title>
    <style>
        :root {{
            --bg-primary: #0d1117;
            --bg-secondary: #161b22;
            --bg-tertiary: #21262d;
            --border: #30363d;
            --text-primary: #e6edf3;
            --text-secondary: #8b949e;
            --text-muted: #6e7681;
            --accent: #58a6ff;
            --accent-hover: #79c0ff;
            --star-color: #f0883e;
            --fork-color: #8b949e;
            --success: #3fb950;
            --font-mono: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, monospace;
            --font-sans: -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif;
        }}
        
        * {{
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }}
        
        body {{
            font-family: var(--font-sans);
            background: var(--bg-primary);
            color: var(--text-primary);
            line-height: 1.6;
            min-height: 100vh;
        }}
        
        .container {{
            max-width: 1400px;
            margin: 0 auto;
            padding: 2rem;
        }}
        
        header {{
            background: var(--bg-secondary);
            border: 1px solid var(--border);
            border-radius: 12px;
            padding: 2rem;
            margin-bottom: 2rem;
        }}
        
        h1 {{
            font-size: 1.75rem;
            font-weight: 600;
            margin-bottom: 0.5rem;
            display: flex;
            align-items: center;
            gap: 0.75rem;
        }}
        
        h1::before {{
            content: '';
            display: inline-block;
            width: 32px;
            height: 32px;
            background: linear-gradient(135deg, var(--accent) 0%, #a371f7 100%);
            border-radius: 8px;
        }}
        
        .meta {{
            color: var(--text-secondary);
            font-size: 0.875rem;
            display: flex;
            gap: 1.5rem;
            flex-wrap: wrap;
        }}
        
        .meta span {{
            display: flex;
            align-items: center;
            gap: 0.375rem;
        }}
        
        .stats {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
            gap: 1rem;
            margin-bottom: 2rem;
        }}
        
        .stat-card {{
            background: var(--bg-secondary);
            border: 1px solid var(--border);
            border-radius: 10px;
            padding: 1.25rem;
            text-align: center;
        }}
        
        .stat-value {{
            font-size: 2rem;
            font-weight: 700;
            font-family: var(--font-mono);
            background: linear-gradient(135deg, var(--accent) 0%, #a371f7 100%);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            background-clip: text;
        }}
        
        .stat-label {{
            font-size: 0.75rem;
            color: var(--text-muted);
            text-transform: uppercase;
            letter-spacing: 0.05em;
            margin-top: 0.25rem;
        }}
        
        .table-wrapper {{
            background: var(--bg-secondary);
            border: 1px solid var(--border);
            border-radius: 12px;
            overflow: hidden;
        }}
        
        table {{
            width: 100%;
            border-collapse: collapse;
        }}
        
        thead {{
            background: var(--bg-tertiary);
        }}
        
        th {{
            padding: 1rem;
            text-align: left;
            font-size: 0.75rem;
            font-weight: 600;
            text-transform: uppercase;
            letter-spacing: 0.05em;
            color: var(--text-secondary);
            border-bottom: 1px solid var(--border);
            position: sticky;
            top: 0;
        }}
        
        th:first-child {{
            width: 60px;
            text-align: center;
        }}
        
        td {{
            padding: 1rem;
            border-bottom: 1px solid var(--border);
            vertical-align: top;
        }}
        
        tr:last-child td {{
            border-bottom: none;
        }}
        
        tr:hover td {{
            background: var(--bg-tertiary);
        }}
        
        .rank {{
            text-align: center;
            font-family: var(--font-mono);
            font-size: 0.875rem;
            color: var(--text-muted);
        }}
        
        .name {{
            font-weight: 600;
            white-space: nowrap;
        }}
        
        .name a {{
            color: var(--accent);
            text-decoration: none;
            display: flex;
            flex-direction: column;
            gap: 0.25rem;
        }}
        
        .name a:hover {{
            color: var(--accent-hover);
        }}
        
        .lang {{
            font-size: 0.7rem;
            font-weight: 500;
            padding: 0.125rem 0.5rem;
            border-radius: 20px;
            background: var(--bg-primary);
            color: var(--text-secondary);
            display: inline-block;
            width: fit-content;
        }}
        
        .lang-python {{ background: rgba(55, 118, 171, 0.2); color: #61dafb; }}
        .lang-javascript {{ background: rgba(247, 223, 30, 0.15); color: #f7df1e; }}
        .lang-typescript {{ background: rgba(49, 120, 198, 0.2); color: #3178c6; }}
        .lang-rust {{ background: rgba(222, 165, 132, 0.2); color: #dea584; }}
        .lang-go {{ background: rgba(0, 173, 216, 0.2); color: #00add8; }}
        .lang-java {{ background: rgba(177, 114, 25, 0.2); color: #b8962c; }}
        .lang-cpp {{ background: rgba(0, 89, 156, 0.2); color: #00599c; }}
        .lang-c {{ background: rgba(165, 165, 165, 0.15); color: #a8a8a8; }}
        .lang-ruby {{ background: rgba(112, 21, 22, 0.2); color: #cc342d; }}
        .lang-php {{ background: rgba(121, 77, 144, 0.2); color: #777bb4; }}
        .lang-swift {{ background: rgba(240, 81, 56, 0.2); color: #f05122; }}
        .lang-kotlin {{ background: rgba(169, 123, 255, 0.2); color: #a97bff; }}
        .lang-shell {{ background: rgba(136, 155, 136, 0.2); color: #89b4fa; }}
        .lang-html {{ background: rgba(227, 101, 36, 0.2); color: #e36524; }}
        .lang-css {{ background: rgba(21, 114, 182, 0.2); color: #1572b6; }}
        
        .description {{
            color: var(--text-secondary);
            font-size: 0.875rem;
            max-width: 400px;
        }}
        
        .stars, .forks {{
            font-family: var(--font-mono);
            font-size: 0.875rem;
            white-space: nowrap;
        }}
        
        .stars {{
            color: var(--star-color);
        }}
        
        .stars::before {{
            content: '★ ';
            opacity: 0.7;
        }}
        
        .forks {{
            color: var(--fork-color);
        }}
        
        .forks::before {{
            content: '⑂ ';
            opacity: 0.5;
        }}
        
        .updated {{
            font-size: 0.8rem;
            color: var(--text-muted);
            white-space: nowrap;
        }}
        
        footer {{
            text-align: center;
            padding: 2rem;
            color: var(--text-muted);
            font-size: 0.8rem;
        }}
        
        footer a {{
            color: var(--accent);
            text-decoration: none;
        }}
        
        @media (max-width: 768px) {{
            .container {{
                padding: 1rem;
            }}
            
            header {{
                padding: 1.25rem;
            }}
            
            h1 {{
                font-size: 1.25rem;
            }}
            
            .stats {{
                grid-template-columns: repeat(2, 1fr);
            }}
            
            .stat-value {{
                font-size: 1.5rem;
            }}
            
            th, td {{
                padding: 0.75rem 0.5rem;
            }}
            
            .description {{
                display: none;
            }}
        }}
        
        @media (prefers-reduced-motion: reduce) {{
            * {{
                transition: none !important;
                animation: none !important;
            }}
        }}
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>{}</h1>
            <div class="meta">
                <span>📊 {} repositories</span>
                <span>🔍 GitHub Search</span>
                <span>📅 Generated {}</span>
            </div>
        </header>
        
        <div class="stats">
            <div class="stat-card">
                <div class="stat-value">{}</div>
                <div class="stat-label">Total Stars</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">{}</div>
                <div class="stat-label">Total Forks</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">{}</div>
                <div class="stat-label">Languages</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">{}</div>
                <div class="stat-label">Repositories</div>
            </div>
        </div>
        
        <div class="table-wrapper">
            <table>
                <thead>
                    <tr>
                        <th>#</th>
                        <th>Repository</th>
                        <th>Description</th>
                        <th>Stars</th>
                        <th>Forks</th>
                        <th>Updated</th>
                    </tr>
                </thead>
                <tbody>
                    {}
                </tbody>
            </table>
        </div>
        
        <footer>
            Generated by <a href="https://github.com/dablon/github-recon" target="_blank">github-recon</a>
        </footer>
    </div>
</body>
</html>"##,
        title,
        title,
        repos.len(),
        now(),
        total_stars,
        total_forks,
        unique_langs,
        repos.len(),
        rows
    )
}

fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn format_stars(n: u64) -> String {
    if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1_000.0)
    } else {
        n.to_string()
    }
}

fn format_number(n: u64) -> String {
    n.to_string().replace(',', "")
}

fn format_date(iso: &str) -> String {
    if iso.len() >= 10 {
        let parts: Vec<&str> = iso.split('-').collect();
        if parts.len() >= 3 {
            let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", 
                         "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
            let month_idx: usize = parts[1].parse().unwrap_or(1) - 1;
            let month = months.get(month_idx).unwrap_or(&"???");
            let day = &parts[2][..2];
            let year = parts[0];
            return format!("{} {}, {}", month, day, year);
        }
    }
    iso.to_string()
}

fn now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    let days = secs / 86400;
    let year = 1970 + days / 365;
    let yday = days % 365;
    let month = yday / 30 + 1;
    let day = yday % 30 + 1;
    format!("{:04}-{:02}-{:02}", year, month, day)
}

fn total_stars(repos: &[GitHubRepository]) -> u64 {
    repos.iter().map(|r| r.stargazers_count).sum()
}

fn total_forks(repos: &[GitHubRepository]) -> u64 {
    repos.iter().map(|r| r.forks_count).sum()
}

fn unique_languages(repos: &[GitHubRepository]) -> usize {
    repos.iter()
        .filter_map(|r| r.language.as_ref())
        .collect::<std::collections::HashSet<_>>()
        .len()
}
