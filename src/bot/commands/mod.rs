pub mod recalculate_stat;
pub mod start;

pub fn setup_commands(router: &mut crate::Router) {
    log::info!("bot: attaching commands");
    start::attach_route(router);
    recalculate_stat::attach_route(router);
}
