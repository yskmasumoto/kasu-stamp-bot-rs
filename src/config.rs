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

fn normalize_toml_keys(value: toml::Value) -> toml::Value {
    match value {
        toml::Value::Table(table) => {
            let mut normalized = toml::Table::new();
            for (key, value) in table {
                normalized.insert(key.to_lowercase(), normalize_toml_keys(value));
            }
            toml::Value::Table(normalized)
        }
        toml::Value::Array(values) => toml::Value::Array(values.into_iter().map(normalize_toml_keys).collect()),
        other => other,
    }
}

fn build_shared_config() -> Result<Config> {
    let mut builder = Config::builder();
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config.toml");
    if config_path.exists() {
        let raw_toml = std::fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;
        let parsed_toml: toml::Value = toml::from_str(&raw_toml)
            .with_context(|| format!("Failed to parse config file as TOML: {}", config_path.display()))?;

        let normalized_toml = toml::to_string(&normalize_toml_keys(parsed_toml)).with_context(|| {
            format!(
                "Failed to normalize config file keys (lowercasing) for: {}",
                config_path.display()
            )
        })?;

        builder = builder.add_source(File::from_str(&normalized_toml, config::FileFormat::Toml));
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
            .context(
                "Failed to deserialize configuration. Make sure DISCORD_TOKEN and SAMURAI_CSV_PATH are set (env vars), or config.toml provides discord_token and samurai_csv_path",
            )?;
        Ok(app_config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    #[test]
    fn load_from_env_uppercase_keys() {
        static ENV_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
        let _guard = ENV_MUTEX
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("lock ENV_MUTEX");

        unsafe {
            std::env::set_var("DISCORD_TOKEN", "dummy_token");
            std::env::set_var("SAMURAI_CSV_PATH", "/tmp/samurai.csv");
        }

        let config = build_shared_config().expect("build config");
        let app_config: AppConfig = config.try_deserialize().expect("deserialize config");

        assert_eq!(app_config.discord_token, "dummy_token");
        assert_eq!(app_config.samurai_csv_path, "/tmp/samurai.csv");

        unsafe {
            std::env::remove_var("DISCORD_TOKEN");
            std::env::remove_var("SAMURAI_CSV_PATH");
        }
    }
}
