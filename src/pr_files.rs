use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::Deserialize;

use crate::structs::AppError;

#[derive(Debug, Deserialize)]
pub struct GitHubPRFilesResponseItem {
    pub sha: String,
    pub filename: String,
    pub status: String,
    pub additions: usize,
    pub deletions: usize,
    pub changes: usize,
    pub raw_url: String,
    pub contents_url: String,
    pub patch: String,
}

pub async fn list_pr_files(
    auth_header: &str,
    owner_and_repo: &str,
    pull_number: usize,
) -> Result<Vec<GitHubPRFilesResponseItem>, AppError> {
    let endpoint_url =
        format!("https://api.github.com/repos/{owner_and_repo}/pulls/{pull_number}/files");

    let client = reqwest::Client::new();
    let request = client
        .get(endpoint_url)
        .header(ACCEPT, "application/vnd.github+json")
        .header(AUTHORIZATION, auth_header)
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header(USER_AGENT, "cli")
        .send();

    let Ok(raw_response) = request.await else {
        return Err(AppError::HttpError);
    };

    let Ok(parsed_response) = raw_response.json::<Vec<GitHubPRFilesResponseItem>>().await else {
        return Err(AppError::MalformedApiResponse);
    };

    Ok(parsed_response)
}
