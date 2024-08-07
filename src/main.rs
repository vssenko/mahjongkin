use teloxide::prelude::*;

mod config;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    pretty_env_logger::init();

    log::info!("Starting Mahjongkin bot...");

    let bot = Bot::new(config::get().tg_token.clone());

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}
