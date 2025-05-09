use serenity::all::ReactionType;
use serenity::model::channel::Message;
use serenity::client::Context;
use log::{info, error};

pub async fn reaction(ctx: &Context, msg: &Message) {
    // リアクションに使うカスタム絵文字の名前
    let target_emoji_name = "kasu"; // :kasu: の名前部分

    // メッセージが送信されたサーバー(Guild)のIDを取得
    // (ダイレクトメッセージでは動作しない)
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => {
            info!("Cannot react in DMs.");
            return;
        }
    };

    // (1) Guild ID を使って、HTTP で Guild 情報を取得
    let http_guild = match ctx.http.get_guild(guild_id).await {
        Ok(g) => g,
        Err(why) => {
            error!("Failed to fetch guild via HTTP: {:?}", why);
            return;
        }
    };
    let guild_name = http_guild.name.clone();

    // (2) Guild の絵文字を取得
    let emoji_opt = http_guild
        .emojis
        .values() // &Emoji のイテレータ
        .find(|e| e.name == target_emoji_name)
        .cloned(); // Emoji をクローン

    if let Some(emoji) = emoji_opt {
        // (3) メッセージにリアクションを追加
        let reaction = ReactionType::from(emoji.clone());
        info!("Found emoji: {} (ID: {})", emoji.name, emoji.id);

        if let Err(why) = msg.react(&ctx.http, reaction).await {
            error!("Error reacting to message {}: {:?}", msg.id, why);
        } else {
            info!(
                "Successfully reacted with :{}: to message {}",
                target_emoji_name, msg.id
            );
        }
    } else {
        error!(
            "Error: Custom emoji ':{}:' not found in guild '{}' (ID: {}).",
            target_emoji_name, guild_name, guild_id
        );
    }
}