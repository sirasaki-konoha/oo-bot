use std::sync::Arc;

use lindera::{
    dictionary::load_dictionary, mode::Mode, segmenter::Segmenter, tokenizer::Tokenizer,
};
use rand::Rng;
use serenity::{
    all::{EmojiId, ReactionType},
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use tracing::{error, info};

mod atsumori;
mod oo;
mod reply;

const STAMP: &str = "<:Omilfy:1489695886773587978>";
const EMOJI_ID: u64 = 1489695886773587978;
const EMOJI_NAME: &str = "Omilfy";
const ERROR_MSG_PROBABILITY: f64 = 0.2;

struct Handler {
    tokenizer: Arc<Mutex<Tokenizer>>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        if msg.mentions_me(&ctx).await.unwrap_or_default() {
            reply::handle_reply(&ctx, &msg).await;
            return;
        }

        atsumori::handle_atsumori(&ctx, &msg).await;
        let count = {
            let tokenizer = self.tokenizer.lock().await;
            let mut tokens = tokenizer.tokenize(&msg.content).unwrap_or_default();
            let mut count = 0;
            for token in tokens.iter_mut() {
                if token
                    .details()
                    .get(7)
                    .map(|r| r.contains("オオ"))
                    .unwrap_or(false)
                {
                    count += 1;
                }
            }
            count
        };

        if count == 0 {
            return;
        }

        // 20%の確率でエラー風メッセージを送る
        if rand::thread_rng().gen_bool(ERROR_MSG_PROBABILITY) {
            let error_msg = oo::build_error_msg(&msg.content);
            if let Err(e) = msg.channel_id.say(&ctx.http, error_msg).await {
                error!("メッセージ送信エラー: {:?}", e);
            }
            return;
        }

        if count == 1 {
            if msg.content.contains("これはおお") {
                if let Err(e) = msg.channel_id.say(&ctx.http, STAMP).await {
                    error!("メッセージ送信エラー: {:?}", e);
                }
                return;
            }

            // 1個ならリアクション
            let emoji = ReactionType::Custom {
                animated: false,
                id: EmojiId::new(EMOJI_ID),
                name: Some(EMOJI_NAME.to_string()),
            };
            if let Err(e) = msg.react(&ctx.http, emoji).await {
                error!("リアクション追加エラー: {:?}", e);
            }
        } else {
            // 2個以上ならスタンプをcount個送信
            let send_msg = (0..count).map(|_| STAMP).collect::<Vec<_>>().join(" ");
            if let Err(e) = msg.channel_id.say(&ctx.http, send_msg).await {
                error!("メッセージ送信エラー: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{}としてログインしました", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let token = std::env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN が .env に設定されていません");
    let dictionary = load_dictionary("embedded://ipadic").unwrap();
    let segmenter = Segmenter::new(Mode::Normal, dictionary, None);
    let tokenizer = Tokenizer::new(segmenter);

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler {
            tokenizer: Arc::new(Mutex::new(tokenizer)),
        })
        .await
        .expect("クライアントの作成に失敗しました");

    if let Err(e) = client.start().await {
        error!("クライアントエラー: {:?}", e);
    }
}
