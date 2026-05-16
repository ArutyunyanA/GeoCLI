use crate::api::ApiConfig;
use crate::config::{CliConfig, Command, RouteRequest, read_route_request};
use crate::services;
use reqwest::Client;
use std::error::Error;

pub async fn run() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv().ok();

    let api = ApiConfig::from_env().map_err(|err| format!("Failed to load API config: {err}"))?;
    let cli = CliConfig::from_env()?;
    let client = Client::new();

    match cli.command {
        Command::Route(request) => process_request(&client, &api, request).await,
        Command::Interactive => run_interactive(&client, &api).await,
    }
}

async fn run_interactive(client: &Client, api: &ApiConfig) -> Result<(), Box<dyn Error>> {
    loop {
        let Some(request) = read_route_request()? else {
            println!("Exit requested. Goodbye!");
            break;
        };

        process_request(client, api, request).await?;
    }

    Ok(())
}

async fn process_request(
    client: &Client,
    api: &ApiConfig,
    request: RouteRequest,
) -> Result<(), Box<dyn Error>> {
    let origin_coords = services::get_coords(client, &request.origin, api).await?;
    let destination_coords = services::get_coords(client, &request.destination, api).await?;

    let routes = services::calculate_routes(
        client,
        origin_coords,
        destination_coords,
        api,
        request.route_type,
    )
    .await?;

    println!(
        "\nCalculated routes for mode {}:\n{routes}",
        request.route_type
    );
    Ok(())
}
