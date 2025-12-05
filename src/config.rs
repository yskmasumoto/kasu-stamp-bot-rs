use anyhow::{Context, Result};
use serde::Deserialize;

/// アプリケーション設定を保持する構造体
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    /// Discord ボットのトークン
    #[serde(rename = "DISCORD_TOKEN")]
    pub discord_token: String,
    
    /// 侍データのCSVファイルパス
    #[serde(rename = "SAMURAI_CSV_PATH")]
    pub samurai_csv_path: String,
}

impl AppConfig {
    /// 環境変数と設定ファイルから設定を読み込む
    /// 
    /// 読み込み優先順位:
    /// 1. 環境変数
    /// 2. config.toml ファイル (オプショナル)
    /// 
    /// # 戻り値
    /// * `Ok(AppConfig)` - 設定の読み込みに成功した場合
    /// * `Err(Error)` - 必要な設定値が見つからない場合
    pub fn load() -> Result<Self> {
        let config = config::Config::builder()
            // オプショナルな設定ファイル (config.toml) を読み込む
            .add_source(config::File::with_name("config").required(false))
            // 環境変数を読み込む (環境変数が優先される)
            .add_source(config::Environment::default())
            .build()
            .context("Failed to build configuration")?;

        config
            .try_deserialize()
            .context("Failed to deserialize configuration. Make sure DISCORD_TOKEN and SAMURAI_CSV_PATH are set")
    }
}
