use config::{Config, Environment};

pub const HOST_KEY: &str = "HOST";

pub fn config() -> Result<Config, crate::server_error::ServerError> {
    let config = Config::builder()
        .set_default(HOST_KEY, 5000)?
        .add_source(Environment::with_prefix("BB_BACKEND"))
        .build()?;

    Ok(config)
}
