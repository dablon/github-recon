use crate::api::GitHubRepository;
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

const PYTHON_SCRIPT: &str = include_str!("../github_recon_excel.py");

pub fn write_xlsx_to_file(repos: &[GitHubRepository], path: &Path, query_name: &str) -> Result<usize> {
    let count = repos.len();

    // Serialize repos to JSON
    let json_data = serde_json::to_string(repos).context("failed to serialize repos")?;

    // Write Python script to temp file
    let py_path = "/tmp/github_recon_excel.py";
    std::fs::write(py_path, PYTHON_SCRIPT).context("failed to write python script")?;

    // Write JSON data to temp file
    let json_path = "/tmp/github_recon_data.json";
    std::fs::write(&json_path, json_data).context("failed to write json data")?;

    // Run Python to generate XLSX
    let output = Command::new("python3")
        .arg(py_path)
        .arg("--json")
        .arg(&json_path)
        .arg("--output")
        .arg(path)
        .arg(query_name)
        .output()
        .context("failed to run python script")?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Python script failed: {}", err);
    }

    // Cleanup
    std::fs::remove_file(py_path).ok();
    std::fs::remove_file(json_path).ok();

    Ok(count)
}

#[cfg(test)]
mod tests {
    // Integration test only - requires python3 with openpyxl
}