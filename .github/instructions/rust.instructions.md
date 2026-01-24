---
applyTo: "**/*.rs"
excludeAgent: ["coding-agent"]
---

# Rust (Serenity Bot) Review Instructions

使用言語: 日本語

## モジュール責務と分割

原則: イベントハンドリング (Entry) と ビジネスロジック (Domain/Logic) を分離する。

推奨:
- **`src/main.rs`**: 設定読み込み、クライアント初期化、イベントループの定義のみに留める
- **`src/config.rs`**: アプリケーション設定の管理 (`config.toml` から読み込む)
- **`src/detect.rs`**: 正規表現や文字列判定などの「純粋関数」ロジック (Serenity/Contextに依存しない)
- **`src/discord.rs`**: APIコール (Reply, React) など `Context` を必要とする副作用
- **`src/table.rs`**: データ構造定義とCSV読み込み/ランダム取得ロジック
- Botのイベントハンドラ (`main.rs` 内) が肥大化した場合、具体的な処理は他モジュールへ委譲する

避ける:
- `EventHandler` トレイトの実装内 (`async fn message`) に数百行のロジックを直書きする
- 正規表現のコンパイルやCSV読み込みをメッセージ受信のたびに行う (都度初期化)

レビュー観点:
- ファイル/関数の行数が不必要に増大していないか
- `Context` を渡す必要のないロジック (文字列判定など) が `Context` に依存していないか
- グローバルな設定や定数がマジックナンバーとして散らばっていないか


## エラーハンドリング / 堅牢性

要求:
- ライブラリ: `anyhow` crate を使用し、文脈 (`.context()`) を付与してエラーを伝播する
- パニック回避: `unwrap()` はテストコードや起動時の絶対安全な箇所以外で使用禁止 (`?` 演算子 or `match/if let` 推奨)
- ログ: `println!` ではなく `log::info!`, `log::error!` を使用する

推奨パターン:
```rust
// 良い例
let val = func().context("Failed to execute func")?;

// 避けるべき例
let val = func().unwrap();
```

## 設定管理

要求:
- ライブラリ: `config` crate を使用して設定ファイル (config.toml) から設定を読み込む
- .env ファイルのサポート: `dotenv` クレートは長年更新されていないため、使用禁止
- 設定の読み込み: `config.toml` のみ

推奨パターン:
```rust
// 設定構造体の定義 (src/config.rs)
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub discord_token: String,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let config = config::Config::builder()
            .add_source(config::File::with_name("config").required(true))
            .build()?;
        config.try_deserialize()
    }
}
```
