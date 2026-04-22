use crate::api::GitHubRepository;
use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;
use xlsxwriter::Workbook;

pub fn write_xlsx_to_file(repos: &[GitHubRepository], path: &Path, query_name: &str) -> Result<usize> {
    let mut workbook = Workbook::new(path);

    // Detect category from query name
    let detected_category = detect_category_from_query(query_name);

    // Results sheet - all repos for this query
    {
        let worksheet = workbook.add_worksheet(Some("Results"))?;
        write_results_sheet(&worksheet, repos, query_name)?;
    }

    // Category-specific sheet if detected
    if let Some(category) = detected_category {
        let sheet_name = sanitize_sheet_name(&category);
        if let Some(worksheet) = workbook.add_worksheet(Some(&sheet_name))? {
            write_category_sheet(&worksheet, repos, &category)?;
        }
    }

    // All Combined sheet
    {
        let worksheet = workbook.add_worksheet(Some("All Combined"))?;
        write_combined_sheet(&worksheet, repos)?;
    }

    workbook.close()?;

    Ok(repos.len())
}

fn write_results_sheet(worksheet: &xlsxwriter::Worksheet, repos: &[GitHubRepository], query_name: &str) -> Result<()> {
    let header_format = worksheet.workbook().add_format()
        .set_bold()
        .set_font_color(0xFFFFFF)
        .set_bg_color(0x1F4E79);

    let title_format = worksheet.workbook().add_format()
        .set_font_size(14)
        .set_bold();

    worksheet.write_string(0, 0, &format!("Query: {}", query_name), &title_format)?;
    worksheet.write_string(1, 0, &format!("Total repositories: {}", repos.len()))?;

    let headers = ["#", "Repository", "URL", "Description", "Stars", "Forks", "Language", "Updated"];
    for (col, h) in headers.iter().enumerate() {
        worksheet.write_string(3, col as u32, h, &header_format)?;
    }

    for (row, repo) in repos.iter().enumerate() {
        let i = (row + 4) as u32;
        worksheet.write_number(i, 0, (row + 1) as f64)?;
        worksheet.write_string(i, 1, &repo.full_name)?;
        worksheet.write_string(i, 2, &repo.html_url)?;
        worksheet.write_string(i, 3, repo.description.as_deref().unwrap_or(""))?;
        worksheet.write_number(i, 4, repo.stargazers_count as f64)?;
        worksheet.write_number(i, 5, repo.forks_count as f64)?;
        worksheet.write_string(i, 6, repo.language.as_deref().unwrap_or("?"))?;
        worksheet.write_string(i, 7, &repo.updated_at)?;
    }

    worksheet.set_column(0, 0, 5.0)?;
    worksheet.set_column(1, 1, 40.0)?;
    worksheet.set_column(2, 2, 55.0)?;
    worksheet.set_column(3, 3, 65.0)?;
    worksheet.set_column(4, 5, 8.0)?;
    worksheet.set_column(6, 6, 12.0)?;
    worksheet.set_column(7, 7, 15.0)?;
    worksheet.freeze_row(3);

    Ok(())
}

fn write_combined_sheet(worksheet: &xlsxwriter::Worksheet, repos: &[GitHubRepository]) -> Result<()> {
    let header_format = worksheet.workbook().add_format()
        .set_bold()
        .set_font_color(0xFFFFFF)
        .set_bg_color(0x1F4E79);

    worksheet.write_string(0, 0, "All Repositories Combined", &header_format)?;
    worksheet.write_string(1, 0, &format!("Total: {} repositories", repos.len()))?;

    let headers = ["#", "Repository", "URL", "Description", "Stars", "Forks", "Language", "Updated"];
    for (col, h) in headers.iter().enumerate() {
        worksheet.write_string(2, col as u32, h, &header_format)?;
    }

    for (row, repo) in repos.iter().enumerate() {
        let i = (row + 3) as u32;
        worksheet.write_number(i, 0, (row + 1) as f64)?;
        worksheet.write_string(i, 1, &repo.full_name)?;
        worksheet.write_string(i, 2, &repo.html_url)?;
        worksheet.write_string(i, 3, repo.description.as_deref().unwrap_or(""))?;
        worksheet.write_number(i, 4, repo.stargazers_count as f64)?;
        worksheet.write_number(i, 5, repo.forks_count as f64)?;
        worksheet.write_string(i, 6, repo.language.as_deref().unwrap_or("?"))?;
        worksheet.write_string(i, 7, &repo.updated_at)?;
    }

    worksheet.set_column(0, 0, 5.0)?;
    worksheet.set_column(1, 1, 40.0)?;
    worksheet.set_column(2, 2, 55.0)?;
    worksheet.set_column(3, 3, 65.0)?;
    worksheet.set_column(4, 5, 8.0)?;
    worksheet.set_column(6, 6, 12.0)?;
    worksheet.set_column(7, 7, 15.0)?;
    worksheet.freeze_row(2);

    Ok(())
}

fn write_category_sheet(worksheet: &xlsxwriter::Worksheet, repos: &[GitHubRepository], category: &str) -> Result<()> {
    let header_format = worksheet.workbook().add_format()
        .set_bold()
        .set_font_color(0xFFFFFF)
        .set_bg_color(0x1F4E79);

    worksheet.write_string(0, 0, &format!("Category: {}", category), &header_format)?;
    worksheet.write_string(1, 0, &format!("Total: {} repositories", repos.len()))?;

    let headers = ["#", "Repository", "URL", "Description", "Stars", "Forks", "Language", "Updated"];
    for (col, h) in headers.iter().enumerate() {
        worksheet.write_string(2, col as u32, h, &header_format)?;
    }

    let mut sorted_repos: Vec<_> = repos.iter().collect();
    sorted_repos.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));

    for (row, repo) in sorted_repos.iter().enumerate() {
        let i = (row + 3) as u32;
        worksheet.write_number(i, 0, (row + 1) as f64)?;
        worksheet.write_string(i, 1, &repo.full_name)?;
        worksheet.write_string(i, 2, &repo.html_url)?;
        worksheet.write_string(i, 3, repo.description.as_deref().unwrap_or(""))?;
        worksheet.write_number(i, 4, repo.stargazers_count as f64)?;
        worksheet.write_number(i, 5, repo.forks_count as f64)?;
        worksheet.write_string(i, 6, repo.language.as_deref().unwrap_or("?"))?;
        worksheet.write_string(i, 7, &repo.updated_at)?;
    }

    worksheet.set_column(0, 0, 5.0)?;
    worksheet.set_column(1, 1, 40.0)?;
    worksheet.set_column(2, 2, 55.0)?;
    worksheet.set_column(3, 3, 65.0)?;
    worksheet.set_column(4, 5, 8.0)?;
    worksheet.set_column(6, 6, 12.0)?;
    worksheet.set_column(7, 7, 15.0)?;
    worksheet.freeze_row(2);

    Ok(())
}

/// Auto-detect category from query string
fn detect_category_from_query(query: &str) -> Option<String> {
    let q = query.to_lowercase();

    let keywords = [
        ("pentest", "Pentest"),
        ("penetration testing", "Pentest"),
        ("vulnerability", "Vulnerability"),
        ("vuln scan", "Vulnerability"),
        ("cve", "Vulnerability"),
        ("bug bounty", "Bug Bounty"),
        ("bounty", "Bug Bounty"),
        ("recon", "Reconnaissance"),
        ("reconnaissance", "Reconnaissance"),
        ("enum", "Reconnaissance"),
        ("network", "Network Discovery"),
        ("port scan", "Network Discovery"),
        ("host discovery", "Network Discovery"),
        ("scanner", "Scanner"),
        ("mcp", "MCP"),
        ("model context", "MCP"),
        ("ai agent", "AI Agent"),
        ("llm agent", "AI Agent"),
        ("autonomous", "AI Agent"),
        ("agentic", "AI Agent"),
        ("automation", "Automation"),
        ("exploit", "Exploit"),
        ("payload", "Exploit"),
        ("security", "Security"),
        ("cyber", "Security"),
    ];

    for (keyword, category) in keywords.iter() {
        if q.contains(keyword) {
            return Some(category.to_string());
        }
    }

    None
}

fn sanitize_sheet_name(name: &str) -> String {
    let name = name.replace(' ', "_").replace('/', "_").replace('\\', "_");
    if name.len() > 31 {
        name.chars().take(31).collect()
    } else {
        name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_category_pentest() {
        assert_eq!(detect_category_from_query("pentest automation"), Some("Pentest".to_string()));
        assert_eq!(detect_category_from_query("AI pentest agent"), Some("Pentest".to_string()));
        assert_eq!(detect_category_from_query("penetration testing framework"), Some("Pentest".to_string()));
    }

    #[test]
    fn test_detect_category_vulnerability() {
        assert_eq!(detect_category_from_query("vulnerability scanner"), Some("Vulnerability".to_string()));
        assert_eq!(detect_category_from_query("cve database"), Some("Vulnerability".to_string()));
        assert_eq!(detect_category_from_query("vuln scan"), Some("Vulnerability".to_string()));
    }

    #[test]
    fn test_detect_category_network() {
        assert_eq!(detect_category_from_query("network scanner"), Some("Network Discovery".to_string()));
        assert_eq!(detect_category_from_query("port scan tool"), Some("Network Discovery".to_string()));
        assert_eq!(detect_category_from_query("host discovery"), Some("Network Discovery".to_string()));
    }

    #[test]
    fn test_detect_category_mcp() {
        assert_eq!(detect_category_from_query("MCP server"), Some("MCP".to_string()));
        assert_eq!(detect_category_from_query("model context protocol"), Some("MCP".to_string()));
    }

    #[test]
    fn test_detect_category_ai_agent() {
        assert_eq!(detect_category_from_query("AI agent autonomous"), Some("AI Agent".to_string()));
        assert_eq!(detect_category_from_query("llm agent"), Some("AI Agent".to_string()));
        assert_eq!(detect_category_from_query("agentic AI"), Some("AI Agent".to_string()));
    }

    #[test]
    fn test_detect_category_unknown() {
        assert_eq!(detect_category_from_query("react redux boilerplate"), None);
        assert_eq!(detect_category_from_query("docker compose"), None);
        assert_eq!(detect_category_from_query("python cli tool"), None);
    }

    #[test]
    fn test_sanitize_sheet_name() {
        assert_eq!(sanitize_sheet_name("Network Discovery"), "Network_Discovery");
        assert_eq!(sanitize_sheet_name("AI Agent"), "AI_Agent");
        assert_eq!(sanitize_sheet_name("Very Long Category Name That Exceeds 31 Characters"), "Very_Long_Category_Name_Th");
    }
}