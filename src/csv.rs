use anyhow::{Context, Result};
use csv::Writer;
use std::path::Path;

use crate::api::GitHubRepository;

#[derive(Debug, Clone, serde::Serialize)]
pub struct RepositoryRecord {
    pub url: String,
    pub name: String,
    pub description: String,
    pub stars: u64,
    pub forks: u64,
    pub last_updated: String,
    pub language: String,
}

impl From<GitHubRepository> for RepositoryRecord {
    fn from(repo: GitHubRepository) -> Self {
        Self {
            url: repo.html_url,
            name: repo.full_name,
            description: repo.description.unwrap_or_default(),
            stars: repo.stargazers_count,
            forks: repo.forks_count,
            last_updated: repo.updated_at,
            language: repo.language.unwrap_or_default(),
        }
    }
}

pub fn write_csv_to_file(repos: &[GitHubRepository], path: &Path) -> Result<usize> {
    let mut writer = Writer::from_path(path)
        .with_context(|| format!("Failed to create CSV file: {:?}", path))?;

    writer.write_record(&[
        "url",
        "name",
        "description",
        "stars",
        "forks",
        "last_updated",
        "language",
    ])?;

    for repo in repos {
        let record = RepositoryRecord::from(repo.clone());
        writer.serialize(&record)?;
    }

    writer.flush()?;
    Ok(repos.len())
}

pub fn write_csv_to_stdout(repos: &[GitHubRepository]) -> Result<usize> {
    let mut writer = Writer::from_writer(std::io::stdout());
    writer.write_record(&[
        "url",
        "name",
        "description",
        "stars",
        "forks",
        "last_updated",
        "language",
    ])?;

    for repo in repos {
        let record = RepositoryRecord::from(repo.clone());
        writer.serialize(&record)?;
    }

    writer.flush()?;
    Ok(repos.len())
}
