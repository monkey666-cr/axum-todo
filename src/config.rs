use serde::Deserialize;

// Web 配置
#[derive(Deserialize)]
pub struct WebConfig {
    pub addr: String,
}

// 应用配置
#[derive(Deserialize)]
pub struct Config {
    pub web: WebConfig,
}

impl Config {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let cfg = config::Config::builder()
            .add_source(config::Environment::with_prefix("TODO"))
            .build()
            .unwrap();

        let app = cfg.try_deserialize::<Config>().unwrap();

        Ok(app)
    }
}
