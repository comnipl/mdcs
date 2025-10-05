pub mod config;
pub mod github_access;

use std::time::Duration;

use clap::Parser;
use config::Config;
use tokio::sync::mpsc;
use tracing::{debug, info, warn,error};

#[dotenvy_macros::load]
#[tokio::main]
async fn main() -> anyhow::Result<()>{
    // tracing_subscriberの初期化
    tracing_subscriber::fmt().init();
    info!("MDCS Controller Created!");

    // Configを環境変数またはコマンド引数から取得
    let env_config = Config::parse();

    // デバッグ用 Configを表示する
    debug!("{:?}",env_config);

    let req_client = reqwest::Client::new();

    let (tx, rx) = mpsc::channel::<()>(32);

    tokio::spawn(async move{get_new_commit_task(&env_config,&req_client,rx).await});

    loop{
        // sleep 3 minutes
        let _ = tokio::time::sleep(Duration::new(10,0)).await;
        let _ = tx.send(()).await;
    }
}

async fn get_new_commit_task(env_config: &Config, req_client: &reqwest::Client,mut mpsc_rx: mpsc::Receiver<()>){
    let mut prev_commit_sha = "".to_string();
    loop{
        mpsc_rx.recv().await;

        let response = match github_access::fetch_commits(env_config, req_client).await{
            Ok(response) => response,
            Err(e) => {
                error!("Error Occured while fetch commit from github: {}",e);
                return;
            }
        };

        let Some(sha_value) = response.get("sha")else{
            warn!("'sha' key was not exist in response.");
            continue;
        };
        println!("{:?}",sha_value.to_string());
        if *sha_value.to_string()!=prev_commit_sha{
            info!("commit updated!");
        }
        prev_commit_sha = sha_value.to_string();
    }
}
