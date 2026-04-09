use rand::Rng;
use serenity::all::{Context, Message};
use tracing::error;

pub async fn handle_reply(ctx: &Context, msg: &Message) {
    if msg.mentions_me(ctx).await.unwrap_or_default() {
        let random_reply = &[
            "これはおおだな",
            "おお",
            "これはおおだな。",
            "おおじゃないが",
            "なにがおおだよ",
        ];

        let reply = random_reply[rand::thread_rng().gen_range(0..random_reply.len())];

        if let Err(e) = msg.channel_id.say(&ctx.http, reply).await {
            error!("メッセージ送信エラー: {:?}", e);
        }
    }
}
