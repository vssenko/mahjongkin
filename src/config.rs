use std::env;
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

pub struct Config {
    pub tg_token: String,
    pub mahjong_tag: String,
    pub mahjong_half_tag: String,
}

pub fn get() -> &'static Config {
    CONFIG.get_or_init(|| {
        let _ = dotenvy::dotenv();
        return Config {
            tg_token: env::var("TG_TOKEN").expect("TG_TOKEN variable is required"),
            mahjong_tag: "#маджонг".into(),
            mahjong_half_tag: "#полханчана".into(),
        };
    })
}
