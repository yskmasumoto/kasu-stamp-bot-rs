# kasu-stamp-bot-rs

これは Rust 言語と `serenity` ライブラリを使用して作成されたシンプルな Discord ボットです。「侍」で終わる日本語のフレーズを含むメッセージを検知し、特定のカスタム絵文字 `:kasu:` で自動的にリアクションを付けます。

## 特徴

* `[任意の文字]侍` というパターンのメッセージ（例: 「ピタッとハウス侍」、「ゲームしたい侍」）を検知します。
* 検知したメッセージにカスタム絵文字 `:kasu:` でリアクションを付けます。
* 必要なイベントを効率的に受信するために Discord Gateway Intents を使用します。
* 設定は `config.toml` から読み込みます。

## 必要条件

* [Rust](https://www.rust-lang.org/ja/): Rust および Cargo (Rust のパッケージマネージャー) をインストールしていること。
* [Discord Bot Token](https://discord.com/developers/applications): Discord アプリケーションを作成し、ボットトークンを取得していること。
* Discord Developer Portal のボット設定で、**`MESSAGE CONTENT`** Privileged Intent を有効にしていること。
* メッセージの読み取りとリアクションの追加の権限を持つように、ボットをサーバーに招待していること。
* ボットを使用するサーバーに、**`:kasu:`** という名前のカスタム絵文字が存在すること。

## セットアップ

1.  **リポジトリをクローンする:**

    ```bash
    git clone https://github.com/yskmasumoto/kasu-stamp-bot-rs.git
    cd kasu-stamp-bot-rs
    ```

2.  **config.toml を作成する:**

    プロジェクトのルートディレクトリに `config.toml` という名前のファイルを作成し、以下のように記述します:
    ```toml
    discord_token = "あなたのボットトークンをここに記述"
    samurai_csv_path = "/path/to/samurai.csv"
    ```

    `あなたのボットトークンをここに記述` を実際のボットトークンに置き換えてください。

    **注意**: `config.toml` にはトークンなどの機密情報が含まれるため、Git にコミットしないでください（このリポジトリでは `.gitignore` で除外しています）。

    **補足**: `config.toml` のキーは小文字の `snake_case` を前提にしています（例: `discord_token`）。キーは大小文字を区別するため、`DISCORD_TOKEN` のような大文字キーは使用しないでください。

3.  **ボットを実行する:**

    Cargo を使用してプロジェクトをビルドし、実行します:

    ```bash
    cargo run
    ```

    ボットが Discord に接続され、接続されたことを示すメッセージが出力されるはずです。

## 設定

* **Discord トークン:** `config.toml` の `discord_token` で設定します。
* **侍CSVパス:** `config.toml` の `samurai_csv_path` で設定します。
    * CSVはヘッダー行が必須で、最低限 `Name` と `Description` 列が必要です。
    * 例: `S_No.,Name,Description` のようなヘッダーを含むCSVを指定してください。
* **対象絵文字:** ボットはカスタム絵文字 `:kasu:` でリアクションするようにハードコードされています。これを変更するには、`src/discord.rs` の `target_emoji_name` 変数を修正する必要があります。
* **検知パターン:** 「侍」で終わるメッセージを検知するためのパターンは、`src/detect.rs` の正規表現で定義されています。

## 動作解説

1.  ボットは `serenity` ライブラリを使用し、提供されたトークンとインテントで Discord に接続します。
2.  参加しているサーバーでの新しいメッセージをリッスンします。
3.  メッセージが受信されると、`src/main.rs` の `message` イベントハンドラがトリガーされます。
4.  メッセージの内容は `src/detect.rs` の `contains_samurai_phrase` 関数に渡されます。
5.  `src/detect.rs` は正規表現 (`[\s\S]+?侍`) を使用して、メッセージに任意の文字に続いて「侍」が含まれているかどうかをチェックします。
6.  フレーズが検知された場合、ボットは HTTP 経由でサーバーのカスタム絵文字リストを取得します。
7.  サーバーの絵文字の中から「kasu」という名前のカスタム絵文字を検索します。
8.  `:kasu:` 絵文字が見つかった場合、ボットはその絵文字を検知したメッセージにリアクションとして追加します。

## 依存関係

このプロジェクトは以下の主要な依存関係を使用しています:

* [`serenity`](https://crates.io/crates/serenity): Discord API 用の Rust ライブラリ。
* [`tokio`](https://crates.io/crates/tokio): Rust のための非同期ランタイム。
* [`async-trait`](https://crates.io/crates/async-trait): トレイトでの非同期関数を容易にするための手続きマクロ。
* [`config`](https://crates.io/crates/config): 設定ファイルから設定を読み込みます。
* [`regex`](https://crates.io/crates/regex): 正規表現のための Rust ライブラリ。
* [`once_cell`](https://crates.io/crates/once_cell): 静的変数の安全な一度だけ代入を実現します。
* [`anyhow`](https://crates.io/crates/anyhow): エラーハンドリングを簡潔にするライブラリ。
* [`csv`](https://crates.io/crates/csv): CSVファイルの読み書きをサポートします。

特定のバージョンや機能については `Cargo.toml` を参照してください。
