use mobot::*;

pub fn attach_route(router: &mut crate::Router) {
    router.add_route(
        Route::Message(Matcher::BotCommand("start".into())),
        |e, _: State<()>| async move {
            Ok(Action::ReplyText(
                "Hello, my name is Mahjonkin and I am here to help you manage your Mahjong stats."
                    .into(),
            ))
        },
    );
}
