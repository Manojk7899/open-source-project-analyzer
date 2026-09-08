use serde::{Serialize,Deserialize};

#[derive(Debug,Deserialize)]
pub struct RepositoryUrlRequest{
    pub url:String
}
#[derive(Debug,Serialize)]
pub struct RepositoryPreviewResponse{
    pub owner:String,
    pub name:String,
    pub normalized_url:String,
}
#[derive(Debug,Serialize)]
pub struct ErrorResponse{
    pub error:String
}   
#[derive(Debug,Deserialize)]
pub struct GitHubOwner{
    pub login:String,
}
#[derive(Debug,Deserialize)]
pub struct GitHubLicense{
    pub spdx_id:String,
}
#[derive(Debug,Deserialize)]
pub struct GitHubRepositoryResponse {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub owner: GitHubOwner,
    pub description: Option<String>,
    pub stargazers_count: i64,
    pub forks_count: i64,
    pub open_issues_count: i64,
    pub subscribers_count: i64,
    pub language: Option<String>,
    pub license: Option<GitHubLicense>,
    pub archived: bool,
    pub has_issues: bool,
    pub html_url: String,
    pub default_branch: String,
    pub topics: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
}

#[derive(Debug, Deserialize)]
pub struct GitHubReadmeResponse {
    pub download_url: String,
}

#[derive(Debug, Serialize)]
pub struct RepositoryFetchResponse {
    pub database_id: i64,
    pub github_id: i64,
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub stars: i64,
    pub forks: i64,
    pub open_issues: i64,
    pub subscribers: i64,
    pub language: Option<String>,
    pub license: Option<String>,
    pub archived: bool,
    pub has_issues: bool,
    pub url: String,
    pub default_branch: String,
    pub topics: Vec<String>,
    pub github_created_at: String,
    pub github_updated_at: String,
    pub pushed_at: String,
    pub readme: String,
}

#[derive(Debug, Serialize)]
pub struct RepositoryListItemResponse {
    pub database_id: i64,
    pub full_name: String,
    pub description: Option<String>,
    pub stars: i64,
    pub forks: i64,
    pub open_issues: i64,
    pub language: Option<String>,
    pub license: Option<String>,
    pub archived: bool,
    pub default_branch: String,
}

#[derive(Debug, Serialize)]
pub struct StoredRepositoryResponse {
    pub database_id: i64,
    pub github_id: i64,
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub stars: i64,
    pub forks: i64,
    pub open_issues: i64,
    pub subscribers: i64,
    pub language: Option<String>,
    pub license: Option<String>,
    pub archived: bool,
    pub has_issues: bool,
    pub url: String,
    pub default_branch: String,
    pub topics: Vec<String>,
    pub github_created_at: String,
    pub github_updated_at: String,
    pub pushed_at: String,
    pub readme: String,
}