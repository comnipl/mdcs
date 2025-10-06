pub mod config;

use clap::Parser;
use config::Config;
use tracing::{debug, info};

#[tokio::main]
async fn main() {
    // tracing_subscriberの初期化
    tracing_subscriber::fmt().init();
    info!("MDCS Controller Created!");

    // Configを環境変数またはコマンド引数から取得
    let config = Config::parse();

    // デバッグ用 Configを表示する
    debug!("{:?}", config);
}
