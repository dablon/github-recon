use crate::api::GitHubRepository;
use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

const PYTHON_SCRIPT: &str = include_str!("../scripts/gen_excel.py");

pub fn write_xlsx_to_file(repos: &[GitHubRepository], path: &Path, query_name: &str) -> Result<usize> {
    let count = repos.len();

    let csv_path = "/tmp/github_recon_data.csv";
    let mut csv_content = String::from("full_name,html_url,description,stargazers_count,forks_count,language,updated_at\n");
    for repo in repos {
        let desc = repo.description.as_deref().unwrap_or("").replace('\n', " ").replace('"', "'");
        csv_content.push_str(&format!(
            "\"{}\",\"{}\",\"{}\",{},{},\"{}\",\"{}\"\n",
            repo.full_name.replace('"', "'"),
            repo.html_url,
            desc,
            repo.stargazers_count,
            repo.forks_count,
            repo.language.as_deref().unwrap_or(""),
            repo.updated_at
        ));
    }
    std::fs::write(&csv_path, csv_content).context("failed to write csv data")?;

    let py_path = "/tmp/gen_excel.py";
    std::fs::write(py_path, PYTHON_SCRIPT).context("failed to write python script")?;

    let output = Command::new("python3")
        .arg(py_path)
        .arg("--csv")
        .arg(&csv_path)
        .arg("--output")
        .arg(path)
        .arg("--query")
        .arg(query_name)
        .output()
        .context("failed to run python")?;

    if !output.status.success() {
        anyhow::bail!("Python failed: {}", String::from_utf8_lossy(&output.stderr));
    }

    std::fs::remove_file(py_path).ok();
    std::fs::remove_file(csv_path).ok();

    Ok(count)
}

#[cfg(test)]
mod tests {
    // Integration test only
}