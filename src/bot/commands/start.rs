use mobot::*;

pub fn attach_route(router: &mut mobot::Router<()>) {
    router.add_route(
        Route::Any(Matcher::BotCommand("start".into())),
        |e, _: State<()>| async move {
            Ok(Action::ReplyText(
                "Hello, my name is Mahjonkin and I am here to help you manage your Mahjong stats."
                    .into(),
            ))
        },
    );
}
