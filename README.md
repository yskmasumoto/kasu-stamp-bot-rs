# kasu-stamp-bot-rs

これは Rust 言語と `serenity` ライブラリを使用して作成された Discord ボットです。

- 「侍」を含むメッセージを検知し、カスタム絵文字 `:kasu:` でリアクションして、過去データからランダムに選んだ侍名を返信します。
- 「ザウルス」を含むメッセージを検知し、Ollama（ローカルLLM）に問い合わせた返答を返信します。

## 特徴

* 「侍」を含むメッセージ（例: 「ピタッとハウス侍」、「ゲームしたい侍」、「侍」）を検知します。
* 検知したメッセージにカスタム絵文字 `:kasu:` でリアクションを付けます。
* 過去データ（CSV）からランダムに侍名を選んで返信します。
* 「ザウルス」を含むメッセージ（例: 「テストザウルス」、「ザウルス」）を検知し、Ollamaの返答を返信します。
* 必要なイベントを効率的に受信するために Discord Gateway Intents を使用します。
* 設定は `config.toml` から読み込みます。
* `RUST_LOG` でログレベルを調整できます（`env_logger`）。

## 必要条件

* [Rust](https://www.rust-lang.org/ja/): Rust および Cargo (Rust のパッケージマネージャー) をインストールしていること。
* [Discord Bot Token](https://discord.com/developers/applications): Discord アプリケーションを作成し、ボットトークンを取得していること。
* Discord Developer Portal のボット設定で、**`MESSAGE CONTENT`** Privileged Intent を有効にしていること。
* メッセージの読み取りとリアクションの追加の権限を持つように、ボットをサーバーに招待していること。
* ボットを使用するサーバーに、**`:kasu:`** という名前のカスタム絵文字が存在すること。
* （ザウルス機能を使う場合）Ollama が起動していること（例: `ollama serve`）。

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

    # Ollama（ザウルス機能）
    default_ollama_base_url = "http://127.0.0.1:11434"
    default_ollama_model = "llama3.2:1b"

    # システムプロンプト（将来拡張用 / 実装に合わせて使用）
    default_system_prompt_path = "/path/to/system_prompt.txt"
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

    ログを詳しく見たい場合:

    ```bash
    RUST_LOG=info,serenity=warn,tracing=warn cargo run
    ```

## 設定

* **Discord トークン:** `config.toml` の `discord_token` で設定します。
* **侍CSVパス:** `config.toml` の `samurai_csv_path` で設定します。
    * CSVはヘッダー行が必須で、最低限 `Name` と `Description` 列が必要です。
    * 例: `S_No.,Name,Description` のようなヘッダーを含むCSVを指定してください。
* **対象絵文字:** ボットはカスタム絵文字 `:kasu:` でリアクションするようにハードコードされています。これを変更するには、`src/discord.rs` の `target_emoji_name` 変数を修正する必要があります。
* **検知パターン:** 検知ルールは `src/detect.rs` の正規表現で定義されています（「侍」「ザウルス」を含むかどうか）。

### Ollama（ザウルス機能）

* **Ollama Base URL:** `config.toml` の `default_ollama_base_url` で設定します。
* **Ollama Model:** `config.toml` の `default_ollama_model` で設定します。
* **System Prompt:** `config.toml` の `default_system_prompt_path` でシステムプロンプトを記述したファイルパスを設定します（未指定時はデフォルト文言を使用）

## 動作解説

1.  ボットは `serenity` を使用し、提供されたトークンとインテントで Discord に接続します。
2.  サーバー内の新しいメッセージを監視します（DM は無視します）。
3.  メッセージが受信されると、`src/main.rs` の `message` イベントハンドラが呼ばれます。
4.  `src/detect.rs` の関数で「侍」「ザウルス」を含むか判定します（例: 正規表現 `[^\n\r]*?侍` ではなく、改行も含めた `"[\s\S]*?侍"` のような判定）。
5.  「侍」の場合: `:kasu:` リアクションを付与し、CSVから侍名をランダムに選んで返信します。
6.  「ザウルス」の場合: Ollama に `/api/chat` で問い合わせ、返答テキストを返信します。

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
* [`reqwest`](https://crates.io/crates/reqwest): Ollama へのHTTPリクエストに使用します。
* [`env_logger`](https://crates.io/crates/env_logger): `RUST_LOG` によるログ出力に使用します。

特定のバージョンや機能については `Cargo.toml` を参照してください。
