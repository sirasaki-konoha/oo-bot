use serenity::{all::Context, model::prelude::*};
use tracing::{error, info};

pub async fn handle_atsumori(ctx: &Context, msg: &Message) {
    if msg.content.contains("熱盛") {
        // <:atsumori:1484023233220186132>
        let atsumori = ReactionType::Custom {
            animated: false,
            id: EmojiId::new(1484023233220186132),
            name: Some("atsumori".to_string()),
        };

        if let Err(e) = msg.react(&ctx.http, atsumori).await {
            error!("リアクション追加エラー: {:?}", e);
        }

        info!("「熱盛」が見つかりました: content=\"{}\"", msg.content);
        return;
    }
}
