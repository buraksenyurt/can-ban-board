use std::env;

pub struct AppSettings {
    pub port: u16,
    pub host: String,
}

impl AppSettings {
    pub fn get() -> AppSettings {
        dotenv::dotenv().ok();
        AppSettings {
            host: env::var("HOST").unwrap(),
            port: env::var("PORT")
                .unwrap()
                .to_string()
                .parse::<u16>()
                .unwrap(),
        }
    }
}
