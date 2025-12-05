# GitHub Copilot Instructions for kasu-stamp-bot-rs

あなたはRustとSerenityを用いたDiscordボット開発のエキスパートとして振る舞ってください。
このプロジェクトは「侍」という語尾に反応してリアクションやリプライをするBotです。

## プロジェクトの技術スタックと制約
- **言語**: Rust (Edition 2024)
- **非同期ランタイム**: `tokio`
- **Discordライブラリ**: `serenity` (v0.12系)
  - ※ v0.12系を使用しているため、v0.11以前やv0.13以降の破壊的変更を含むコードを提案しないでください。
- **エラーハンドリング**: `anyhow` crate を使用する。
- **静的変数管理**: `once_cell::sync::Lazy` を使用する。
- **CSV処理**: `csv` crate を使用し、`src/table.rs` で管理する。

## コーディングスタイル
- **コメント**: コード内のコメントは全て「日本語」で記述すること。
- **モジュール構成**:
  - `src/detect.rs`: 文字列判定ロジック（正規表現 `regex` 使用）
  - `src/discord.rs`: Discord APIとの通信（リアクション、リプライ）
  - `src/table.rs`: CSVデータの読み込みとランダム取得
  - `src/main.rs`: エントリーポイント、イベントハンドラ
- **ログ**: `log` crate (`info!`, `error!` 等) を使用して適切に出力する。

## 挙動の前提
- Botは `DISCORD_TOKEN` 環境変数で動作する。
- `.env` ファイルを利用する。
- 特定の絵文字 `:kasu:` をリアクションに使用する。
