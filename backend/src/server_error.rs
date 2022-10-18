#[derive(Debug)]
pub enum ServerError {
    ConfigError(config::ConfigError),
    HttpServerError(std::io::Error),
}

impl From<config::ConfigError> for ServerError {
    fn from(error: config::ConfigError) -> Self {
        Self::ConfigError(error)
    }
}
impl From<std::io::Error> for ServerError {
    fn from(error: std::io::Error) -> Self {
        Self::HttpServerError(error)
    }
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::ConfigError(ce) => write!(f, "{}", ce),
            ServerError::HttpServerError(se) => write!(f, "{}", se),
        }
    }
}

impl std::error::Error for ServerError {}
