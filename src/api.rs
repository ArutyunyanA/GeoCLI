use std::env;

#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub token: String,
    pub geocode_url: String,
    pub directions_url: String,
}

impl ApiConfig {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            token: env::var("TOKEN")?,
            geocode_url: env::var("GEOCODE")
                .unwrap_or_else(|_| "https://api.mapbox.com/search/geocode/v6/forward".to_string()),
            directions_url: env::var("DIRECTIONS")
                .unwrap_or_else(|_| "https://api.mapbox.com/directions/v5/mapbox".to_string()),
        })
    }
}
