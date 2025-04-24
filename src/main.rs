use serenity::all::ReactionType;
use serenity::async_trait;
use serenity::model::{channel::Message, gateway::Ready, prelude::*};
use serenity::prelude::*;
use std::env;

// イベントハンドラ用構造体
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // メッセージが作成されたときに呼ばれる関数
    async fn message(&self, ctx: Context, msg: Message) {
        // メッセージの内容が "侍" と完全に一致する場合
        if msg.content.contains("侍") {
            println!("Received '侍' from user: {}", msg.author.name); // デバッグ用ログ

            // リアクションに使うカスタム絵文字の名前
            let target_emoji_name = "kasu"; // :kasu: の名前部分

            // メッセージが送信されたサーバー(Guild)のIDを取得
            // (ダイレクトメッセージでは動作しない)
            let guild_id = match msg.guild_id {
                Some(id) => id,
                None => {
                    println!("Cannot react in DMs.");
                    return;
                }
            };

            // — キャッシュ取得 ↓ を HTTP 取得に置き換え —
            let http_guild = match ctx.http.get_guild(guild_id).await {
                Ok(g) => g,
                Err(why) => {
                    eprintln!("Failed to fetch guild via HTTP: {:?}", why);
                    return;
                }
            };
            let guild_name = http_guild.name.clone();

            // (1) Emoji だけを取り出す
            let emoji_opt = http_guild
                .emojis
                .values() // &Emoji のイテレータ
                .find(|e| e.name == target_emoji_name)
                .cloned(); // Emoji をクローン

            if let Some(emoji) = emoji_opt {
                // (2) そのまま Emoji を From できる
                let reaction = ReactionType::from(emoji.clone());
                println!("Found emoji: {} (ID: {})", emoji.name, emoji.id);

                if let Err(why) = msg.react(&ctx.http, reaction).await {
                    eprintln!("Error reacting to message {}: {:?}", msg.id, why);
                } else {
                    println!(
                        "Successfully reacted with :{}: to message {}",
                        target_emoji_name, msg.id
                    );
                }
            } else {
                eprintln!(
                    "Error: Custom emoji ':{}:' not found in guild '{}' (ID: {}).",
                    target_emoji_name, guild_name, guild_id
                );
            }
        }
    }

    // ボットが起動し、準備ができたときに呼ばれる関数
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
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

    // --- ボットの起動 ---
    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
