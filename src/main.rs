use serenity::async_trait;
use serenity::model::{channel::Message, gateway::Ready, prelude::*};
use serenity::prelude::*;
use std::env;
use once_cell::sync::Lazy;
mod detect;
mod table;
mod discord;
use polars::prelude::*;
use log::{info, error};

// イベントハンドラ用構造体
struct Handler;

// テーブル
static SAMURAI_DATA: Lazy<Result<DataFrame, PolarsError>> = Lazy::new(|| table::read_samurai_csv());

#[async_trait]
impl EventHandler for Handler {
    /// メッセージが送信されたときに呼ばれる関数
    /// # 引数
    /// * `ctx` - コンテキスト (メッセージの送信先やボットの情報など)
    /// * `msg` - 送信されたメッセージ
    async fn message(&self, ctx: Context, msg: Message) {
        // --- メッセージの内容から侍を検出 ---
        let is_samurai = detect::contains_samurai_phrase(&msg.content);

        if is_samurai {
            info!("Received '侍' from user: {}", msg.author.name);

            // --- メッセージの内容に応じてリアクションを実行 ---
            discord::reaction(&ctx, &msg).await;

            // --- SAMURAI_DATAからDataFrameを取得 ---
            let df = match &*SAMURAI_DATA {
                Ok(data) => data,
                Err(e) => {
                    error!("Error: {}", e);
                    return;
                }
            };
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
    // --- トークンの設定 ---
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

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
