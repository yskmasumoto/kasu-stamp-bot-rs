use anyhow::{Context, Result};
use serde::Deserialize;

/// アプリケーション設定を保持する構造体
/// 
/// この構造体は `config` クレートを使用して環境変数や設定ファイルから設定を読み込みます。
/// 設定の読み込み優先順位は: 環境変数 > config.toml ファイル
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    /// Discord ボットのトークン
    #[serde(rename = "DISCORD_TOKEN")]
    pub discord_token: String,
    
    /// 侍データのCSVファイルパス
    /// 
    /// 注意: このフィールドは現在、src/table.rs の静的初期化で直接環境変数から読み込まれています。
    /// AppConfig::load() を呼び出すことで、このフィールドに設定ファイルや環境変数から値が設定されます。
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
    /// 読み込んだ設定値は環境変数としても設定されるため、
    /// 他のモジュール (table.rs など) からも std::env::var で参照できます。
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

        let app_config: Self = config
            .try_deserialize()
            .context("Failed to deserialize configuration. Make sure DISCORD_TOKEN and SAMURAI_CSV_PATH are set")?;

        // 他のモジュールが std::env::var で参照できるように環境変数として設定
        // (静的初期化される table.rs の SAMURAI_DATA 用)
        unsafe {
            if std::env::var("DISCORD_TOKEN").is_err() {
                std::env::set_var("DISCORD_TOKEN", &app_config.discord_token);
            }
            if std::env::var("SAMURAI_CSV_PATH").is_err() {
                std::env::set_var("SAMURAI_CSV_PATH", &app_config.samurai_csv_path);
            }
        }

        Ok(app_config)
    }
}
