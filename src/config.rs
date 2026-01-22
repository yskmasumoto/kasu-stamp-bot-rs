use anyhow::{Context, Result, anyhow};
use config::{Config, File};
use once_cell::sync::OnceCell;
use serde::Deserialize;
use std::path::PathBuf;

/// アプリケーション設定を保持する構造体
///
/// この構造体は `config` クレートを使用して設定ファイルから設定を読み込みます。
/// 設定は `config.toml` ファイルから読み込みます。
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    /// Discord ボットのトークン
    pub discord_token: String,

    /// 侍データのCSVファイルパス
    ///
    /// 読み込み元:
    /// - config.toml
    pub samurai_csv_path: String,

    /// ollama サーバーのベースURL
    pub default_ollama_base_url: String,

    /// 使用する ollama モデル名
    pub default_ollama_model: String,

    /// システムプロンプトのファイルパス
    pub default_system_prompt_path: String,
}

fn build_shared_config() -> Result<Config> {
    let mut builder = Config::builder();
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config.toml");
    builder = builder.add_source(File::from(config_path).required(true));
    builder
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
    /// 設定ファイルから設定を読み込む
    ///
    /// 設定ファイル:
    /// 1. config.toml ファイル
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
            .context(
                "Failed to deserialize configuration. Make sure config.toml provides toml keys matching AppConfig fields.",
            )?;
        Ok(app_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_from_toml_lowercase_keys() {
        let raw_toml = r#"
        discord_token = "token_lower"
        samurai_csv_path = "/path/lower.csv"
        "#;

        let config = Config::builder()
            .add_source(File::from_str(raw_toml, config::FileFormat::Toml))
            .build()
            .expect("build config");
        let app_config: AppConfig = config.try_deserialize().expect("deserialize AppConfig");

        assert_eq!(app_config.discord_token, "token_lower");
        assert_eq!(app_config.samurai_csv_path, "/path/lower.csv");
    }
}
