use anyhow::Error;
use once_cell::sync::Lazy;
use serenity::async_trait;
use serenity::model::{channel::Message, gateway::Ready, prelude::*};
use serenity::prelude::*;
mod detect;
mod discord;
mod table;
mod config;
use log::{error, info};
use table::SamuraiEntry;

// イベントハンドラ用構造体
struct Handler;

// テーブル
static SAMURAI_DATA: Lazy<Result<Vec<SamuraiEntry>, Error>> =
    Lazy::new(table::read_samurai_csv_as_vec);

#[async_trait]
impl EventHandler for Handler {
    /// メッセージが送信されたときに呼ばれる関数
    /// # 引数
    /// * `ctx` - コンテキスト (メッセージの送信先やボットの情報など)
    /// * `msg` - 送信されたメッセージ
    async fn message(&self, ctx: Context, msg: Message) {
        // --- メッセージがボットからのものであれば無視 ---
        if msg.author.bot {
            return;
        }

        // --- メッセージがダイレクトメッセージであれば無視 ---
        if msg.guild_id.is_none() {
            info!("Received DM from user: {}", msg.author.name);
            return;
        }

        // --- メッセージの内容から侍を検出 ---
        let is_samurai = detect::contains_samurai_phrase(&msg.content);

        if is_samurai {
            info!("Received '侍' from user: {}", msg.author.name);

            // --- メッセージの内容に応じてリアクションを実行 ---
            discord::samurai_reaction(&ctx, &msg).await;

            // --- SAMURAI_DATAからDataFrameを取得 ---
            let df = match &*SAMURAI_DATA {
                Ok(data) => data,
                Err(e) => {
                    error!("Error: {}", e);
                    return;
                }
            };

            // --- ランダムな侍を過去データから取得 ---
            let sname_res = table::get_samurai_name(df); // Samurai ID を取得
            let sname = match sname_res {
                Ok(Some(name)) => name,
                Ok(None) => {
                    error!("Samurai not found");
                    return;
                }
                Err(e) => {
                    error!("Error: {}", e);
                    return;
                }
            };
            info!("Samurai name: {}", sname);

            // --- メッセージにリプライ ---
            discord::samurai_reply(&ctx, &msg, &sname).await;
            info!("Replied to message: {}", msg.id);
        }
    }

    /// ボットが起動したときに呼ばれる関数
    /// # 引数
    /// * `ctx` - コンテキスト (メッセージの送信先やボットの情報など)
    /// * `ready` - ボットの準備が完了したことを示す情報（ユーザー名など）
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // --- 設定の読み込み ---
    // 環境変数またはconfig.tomlファイルから設定を読み込む
    let app_config = config::AppConfig::load().expect("Failed to load configuration");
    let token = app_config.discord_token;

    // --- インテントの設定 ---
    // ボットが必要とする権限(Intents)を設定する
    let intents = GatewayIntents::GUILD_MESSAGES // サーバー内のメッセージ受信
        | GatewayIntents::MESSAGE_CONTENT   // ★メッセージの内容を読む (特権インテント)
        | GatewayIntents::GUILDS            // サーバー情報の取得 (キャッシュや絵文字検索に必要)
        | GatewayIntents::GUILD_EMOJIS_AND_STICKERS; // サーバーの絵文字リスト取得

    // --- クライアントの構築 ---
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler) // 作成したイベントハンドラを設定
        .await
        .expect("Error creating client");

    // --- 利用データの取得 ---
    match &*SAMURAI_DATA {
        Ok(data) => info!("{:?}", data),
        Err(e) => error!("Error: {}", e),
    }

    // --- ボットの起動 ---
    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
