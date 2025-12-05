use anyhow::{Context, Result, anyhow};
use config::{Config, Environment, File};
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::path::PathBuf;

/// アプリケーション設定を保持する構造体
///
/// この構造体は `config` クレートを使用して環境変数や設定ファイルから設定を読み込みます。
/// 設定の読み込み優先順位は: 環境変数 > config.toml ファイル
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    /// Discord ボットのトークン
    pub discord_token: String,

    /// 侍データのCSVファイルパス
    ///
    /// 読み込み優先順位（後に追加されたソースが優先）:
    /// 1. config.toml ファイル (オプショナル、基本設定)
    /// 2. 環境変数 (上書き、最優先)
    pub samurai_csv_path: String,
}

fn build_shared_config() -> Result<Config> {
    let mut builder = Config::builder();
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config.toml");
    if config_path.exists() {
        builder = builder.add_source(File::from(config_path));
    }
    builder
        .add_source(Environment::default())
        .build()
        .context("Failed to build configuration for the samurai bot")
}

static APP_CONFIG: OnceCell<AppConfig> = OnceCell::new();

/// Configuration that must be initialized at the start of the binary and
/// made available globally via `app_config()`.
pub fn init_app_config() -> Result<&'static AppConfig> {
    let loaded_app_config = AppConfig::load()?;
    APP_CONFIG
        .set(loaded_app_config)
        .map_err(|_| anyhow!("App configuration has already been initialized"))?;
    Ok(APP_CONFIG
        .get()
        .expect("App configuration has not been initialized before use"))
}

/// Accessor for the globally stored configuration.
pub fn app_config() -> &'static AppConfig {
    APP_CONFIG
        .get()
        .expect("App configuration has not been initialized before use")
}

impl AppConfig {
    /// 環境変数と設定ファイルから設定を読み込む
    ///
    /// 読み込み優先順位:
    /// 1. 環境変数
    /// 2. config.toml ファイル (オプショナル)
    ///
    /// 読み込んだ設定値は `config::init_app_config()` を通じてグローバルに公開され、
    /// 他のモジュールから `config::app_config()` で取得できます。
    ///
    /// # 戻り値
    /// * `Ok(AppConfig)` - 設定の読み込みに成功した場合
    /// * `Err(Error)` - 必要な設定値が見つからない場合
    pub fn load() -> Result<Self> {
        let config = build_shared_config()?;
        let app_config: Self = config
            .try_deserialize()
            .context("Failed to deserialize configuration. Make sure DISCORD_TOKEN and SAMURAI_CSV_PATH are set")?;
        Ok(app_config)
    }
}
