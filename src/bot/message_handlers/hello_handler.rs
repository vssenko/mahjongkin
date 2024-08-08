use mobot::*;

pub fn attach_handler(router: &mut crate::Router) {
    router.add_route(
        Route::Message(Matcher::Regex("[Пп]ривет|[Зз]дарова".into())),
        |_e, _s| async move { Ok(Action::ReplyText("ЗДАРОВА ПИДРИЛА!".into())) },
    );
}
