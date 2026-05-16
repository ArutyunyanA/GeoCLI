use crate::api::ApiConfig;
use reqwest::Client;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum RouteType {
    Driving,
    Cycling,
    Walking,
}

impl RouteType {
    pub fn profile(self) -> &'static str {
        match self {
            Self::Driving => "driving-traffic",
            Self::Cycling => "cycling",
            Self::Walking => "walking",
        }
    }
}

impl fmt::Display for RouteType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Driving => write!(f, "Driving"),
            Self::Cycling => write!(f, "Cycling"),
            Self::Walking => write!(f, "Walking"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinates {
    lon: f64,
    lat: f64,
}

impl Coordinates {
    fn new(lon: f64, lat: f64) -> Self {
        Self { lon, lat }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("No coordinates found for address: {0}")]
    CoordinatesNotFound(String),
    #[error("No routes found")]
    RoutesNotFound,
}

#[derive(Debug, Deserialize)]
struct GeocodeResponse {
    features: Vec<Feature>,
}

#[derive(Debug, Deserialize)]
struct Feature {
    geometry: Geometry,
}

#[derive(Debug, Deserialize)]
struct Geometry {
    coordinates: [f64; 2],
}

#[derive(Debug, Deserialize)]
struct DirectionsResponse {
    routes: Vec<Route>,
}

#[derive(Debug, Deserialize)]
struct Route {
    distance: f64,
    duration: f64,
}

pub async fn get_coords(
    client: &Client,
    address: &str,
    cfg: &ApiConfig,
) -> Result<Coordinates, ServiceError> {
    let geocode: GeocodeResponse = client
        .get(&cfg.geocode_url)
        .query(&[("q", address), ("access_token", cfg.token.as_str())])
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let coordinates = geocode
        .features
        .first()
        .map(|feature| {
            Coordinates::new(
                feature.geometry.coordinates[0],
                feature.geometry.coordinates[1],
            )
        })
        .ok_or_else(|| ServiceError::CoordinatesNotFound(address.to_owned()))?;

    Ok(coordinates)
}

pub async fn calculate_routes(
    client: &Client,
    start: Coordinates,
    end: Coordinates,
    cfg: &ApiConfig,
    mode: RouteType,
) -> Result<String, ServiceError> {
    let url = format!(
        "{}/{}/{},{};{},{}",
        cfg.directions_url,
        mode.profile(),
        start.lon,
        start.lat,
        end.lon,
        end.lat
    );
    let directions: DirectionsResponse = client
        .get(url)
        .query(&[("access_token", cfg.token.as_str())])
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    if directions.routes.is_empty() {
        return Err(ServiceError::RoutesNotFound);
    }
    let mut result = String::new();
    for (index, route) in directions.routes.iter().enumerate() {
        let distance_km = route.distance / 1000.0;
        let hours = (route.duration / 3600.0).floor() as u64;
        let minutes = ((route.duration % 3600.0) / 60.0).floor() as u64;
        let seconds = (route.duration % 60.0).floor() as u64;

        result.push_str(&format!(
            "Route #{:>2}: {:.2} km | {:02}h {:02}m {:02}s\n",
            index + 1,
            distance_km,
            hours,
            minutes,
            seconds
        ));
    }

    Ok(result)
}
