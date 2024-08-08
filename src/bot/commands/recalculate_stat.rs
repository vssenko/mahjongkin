use api::SendMessageRequest;
use mobot::*;

pub fn attach_route(router: &mut crate::Router) {
    router.add_route(
        Route::Message(Matcher::BotCommand("recalculate_stat".into())),
        |e: Event, _: State<()>| async move {
            let chat_id = e.update.chat_id()?;

            e.api
                .send_message(&SendMessageRequest {
                    chat_id,
                    text: "Started recalculation...".into(),
                    ..Default::default()
                })
                .await?;

            Ok(Action::ReplyText(
                "Here will be your results (TODO).".into(),
            ))
        },
    );
}
