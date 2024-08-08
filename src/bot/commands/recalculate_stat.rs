use api::SendMessageRequest;
use mobot::*;

use crate::bot::api::search::{search_messages, SearchMessagesRequest};
use crate::config;

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

            //This will not work until Telegram API will give any ability to search (at least) with hashtags
            let mahjong_messages = search_messages(
                &e.api,
                SearchMessagesRequest {
                    chat_id,
                    q: config::get().mahjong_tag.clone(),
                },
            )
            .await?;

            log::info!("{:?}", mahjong_messages);

            Ok(Action::ReplyText(
                "Here will be your results (TODO).".into(),
            ))
        },
    );
}
