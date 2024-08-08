use mobot::*;

pub mod hello_handler;

pub fn setup_handlers(router: &mut Router<()>) {
    log::info!("bot: attaching message handlers");
    hello_handler::attach_handler(router);
}
