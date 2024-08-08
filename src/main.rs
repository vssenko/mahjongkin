use mobot::*;

mod bot;
mod config;

pub type Router = mobot::Router<()>;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    pretty_env_logger::init();

    log::info!("Starting Mahjongkin bot...");

    let client = mobot::Client::new(config::get().tg_token.clone());
    let mut router = mobot::Router::new(client);

    bot::commands::setup_commands(&mut router);
    bot::message_handlers::setup_handlers(&mut router);

    router.start().await;
}
