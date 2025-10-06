pub mod config;
pub mod github_access;

use std::time::Duration;

use clap::Parser;
use config::Config;
use tracing::{debug, error, info, warn};

#[dotenvy_macros::load]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // tracing_subscriberの初期化
    tracing_subscriber::fmt().init();
    info!("MDCS Controller Created!");

    // Configを環境変数またはコマンド引数から取得
    let env_config = Config::parse();

    // デバッグ用 Configを表示する
    debug!("{:?}", env_config);

    let req_client = reqwest::Client::new();

    tokio::spawn(async move { get_new_commit_task(&env_config, &req_client).await });

    std::future::pending::<()>().await;
    Ok(())
}

async fn get_new_commit_task(env_config: &Config, req_client: &reqwest::Client) {
    let mut prev_commit_sha = "".to_string();
    loop {
        // sleep 3 minutes
        let _ = tokio::time::sleep(Duration::new(60 * 3, 0)).await;

        let response = match github_access::fetch_commits(env_config, req_client).await {
            Ok(response) => response,
            Err(e) => {
                error!("Error Occured while fetch commit from github: {}", e);
                continue;
            }
        };

        let latest_commit = if response.is_array() {
            response.get(0)
        } else {
            Some(&response)
        };

        let Some(sha_value) = latest_commit
            .and_then(|commit| commit.get("sha"))
            .and_then(|sha| sha.as_str())
        else {
            warn!("Could not extract commit 'sha' from response.");
            continue;
        };
        if sha_value != prev_commit_sha {
            info!("commit updated! new SHA: {}", sha_value);
        }
        prev_commit_sha = sha_value.to_string();
    }
}
