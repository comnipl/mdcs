use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use tracing::error;
use crate::config::Config;

pub async fn fetch_commits(config: &Config, req_client: &reqwest::Client)->anyhow::Result<serde_json::Value>{
    // modify header value
    let mut headers = HeaderMap::new();

    let Ok(auth_header_value) = HeaderValue::from_str(&format!("Bearer {}", config.github_password))else{
        error!("GITHUB_PASSWORD value was invalid.");
        return Err(anyhow::anyhow!("GITHUB_PASSWORD value was invalid."));
    };
    headers.insert(AUTHORIZATION, auth_header_value);
    let accept_header_value = HeaderValue::from_static("application/vnd.github+json");
    headers.insert(ACCEPT,accept_header_value);
    let agent_header_value = HeaderValue::from_static("mdcs-controller");
    headers.insert(USER_AGENT,agent_header_value);
    let gh_version_header_value = HeaderValue::from_static("2022-11-28");
    headers.insert("X-GitHub-Api-Version",gh_version_header_value);

    // create request url
    let url:String;
    if let Some(branch_name) = &config.github_branch_name{
        url = format!("https://api.github.com/repos/{}/{}/commits/{}",config.github_repository_owner_name,config.github_repository_name,branch_name);
    }else{
        url = format!("https://api.github.com/repos/{}/{}/commits",config.github_repository_owner_name,config.github_repository_name);
    };

    // request to github api
    let response = req_client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    Ok(response)
}
