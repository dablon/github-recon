use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

const GITHUB_API_URL: &str = "https://api.github.com";
const MAX_PER_PAGE: usize = 100;

#[derive(Debug, Clone)]
pub struct SearchQuery {
    pub query: String,
    pub page: u32,
    pub per_page: usize,
    pub sort: Option<String>,
    pub order: Option<String>,
}

impl SearchQuery {
    pub fn new(query: &str) -> Self {
        Self {
            query: query.to_string(),
            page: 1,
            per_page: MAX_PER_PAGE,
            sort: None,
            order: None,
        }
    }

    pub fn with_pagination(mut self, page: u32, per_page: usize) -> Self {
        self.page = page;
        self.per_page = per_page.min(MAX_PER_PAGE);
        self
    }

    pub fn with_sort(mut self, sort: impl Into<String>, order: impl Into<String>) -> Self {
        self.sort = Some(sort.into());
        self.order = Some(order.into());
        self
    }

    pub fn to_query_string(&self) -> String {
        self.query.clone()
    }
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub total_count: u64,
    pub incomplete_results: bool,
    pub items: Vec<GitHubRepository>,
}

#[derive(Debug, Clone)]
pub struct GitHubClient {
    client: Client,
    token: Option<String>,
}

impl GitHubClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("github-recon/0.1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            token: std::env::var("GITHUB_TOKEN").ok(),
        }
    }

    pub fn with_token(token: impl Into<String>) -> Self {
        let mut client = Self::new();
        client.token = Some(token.into());
        client
    }

    pub async fn search_repositories(&self, query: &SearchQuery) -> Result<SearchResponse> {
        let query_str = query.to_query_string();
        let url = format!("{}/search/repositories", GITHUB_API_URL);

        let mut request = self.client.get(&url)
            .query(&[
                ("q", &query_str),
                ("page", &query.page.to_string()),
                ("per_page", &query.per_page.to_string()),
            ]);

        if let (Some(sort), Some(order)) = (&query.sort, &query.order) {
            request = request.query(&[("sort", sort.as_str()), ("order", order.as_str())]);
        }

        if let Some(ref token) = self.token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        let response = request.send().await
            .with_context(|| format!("Failed to search repositories with query: {}", query_str))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("GitHub API error ({}): {}", status, body);
        }

        let search_response: SearchResponse = response.json().await
            .with_context(|| "Failed to parse GitHub API response")?;

        Ok(search_response)
    }

    pub async fn search_all_repositories(
        &self,
        query: &SearchQuery,
        max_results: Option<usize>,
    ) -> Result<Vec<GitHubRepository>> {
        let mut all_repos = Vec::new();
        let mut page = query.page;
        let per_page = query.per_page.min(MAX_PER_PAGE);
        let max_results = max_results.unwrap_or(usize::MAX);

        loop {
            let search_query = query.clone()
                .with_pagination(page, per_page);

            let response = self.search_repositories(&search_query).await?;

            let items_len = response.items.len();
            for repo in response.items {
                if all_repos.len() >= max_results {
                    return Ok(all_repos);
                }
                all_repos.push(repo);
            }

            if items_len == 0 || all_repos.len() >= max_results {
                break;
            }

            page += 1;
        }

        Ok(all_repos)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubRepository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub stargazers_count: u64,
    pub forks_count: u64,
    pub language: Option<String>,
    pub updated_at: String,
    pub fork: bool,
}
