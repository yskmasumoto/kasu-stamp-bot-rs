# kasu-stamp-bot-rs

これは Rust 言語と `serenity` ライブラリを使用して作成されたシンプルな Discord ボットです。「侍」で終わる日本語のフレーズを含むメッセージを検知し、特定のカスタム絵文字 `:kasu:` で自動的にリアクションを付けます。

## 特徴

* `[任意の文字]侍` というパターンのメッセージ（例: 「ピタッとハウス侍」、「ゲームしたい侍」）を検知します。
* 検知したメッセージにカスタム絵文字 `:kasu:` でリアクションを付けます。
* 必要なイベントを効率的に受信するために Discord Gateway Intents を使用します。
* セキュリティのため、Discord ボットトークンを `.env` ファイルから読み込みます。

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

2.  **`.env` ファイルを作成する:**

    プロジェクトのルートディレクトリに `.env` という名前のファイルを作成し、Discord ボットトークンを記述します:

    ```dotenv
    DISCORD_TOKEN=あなたのボットトークンをここに記述
    ```

    `あなたのボットトークンをここに記述` を実際のボットトークンに置き換えてください。

3.  **ボットを実行する:**

    Cargo を使用してプロジェクトをビルドし、実行します:

    ```bash
    cargo run
    ```

    ボットが Discord に接続され、接続されたことを示すメッセージが出力されるはずです。

## 設定

* **Discord トークン:** `.env` ファイルの `DISCORD_TOKEN` 環境変数で設定します。
* **対象絵文字:** ボットはカスタム絵文字 `:kasu:` でリアクションするようにハードコードされています。これを変更するには、`src/main.rs` の `target_emoji_name` 変数を修正する必要があります。
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
* [`dotenv`](https://crates.io/crates/dotenv): `.env` ファイルから環境変数を読み込みます。
* [`regex`](https://crates.io/crates/regex): 正規表現のための Rust ライブラリ。
* [`once_cell`](https://crates.io/crates/once_cell): 静的変数の安全な一度だけ代入を実現します。

特定のバージョンや機能については `Cargo.toml` を参照してください。
