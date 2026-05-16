use crate::services::RouteType;
use crate::ui::print_help;

use std::env;
use std::fmt;

#[derive(Debug, Clone)]
pub struct CliConfig {
    pub command: Command,
}

#[derive(Debug, Clone)]
pub enum Command {
    Route(RouteRequest),
    Interactive,
}

#[derive(Debug, Clone)]
pub struct RouteRequest {
    pub origin: String,
    pub destination: String,
    pub route_type: RouteType,
}

#[derive(Debug)]
pub enum ConfigError {
    MissingValue { flag: &'static str },
    MissingRequired(&'static str),
    UknownFlag(String),
    InvalidMode(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingValue { flag } => write!(f, "Missing value for flag {flag}"),
            Self::MissingRequired(flag) => write!(f, "Missing required flag {flag}"),
            Self::UknownFlag(flag) => write!(f, "Unknown option {flag}"),
            Self::InvalidMode(mode) => {
                write!(
                    f,
                    "Invalid route mode '{mode}'. Use: driving | cycling | walking"
                )
            }
        }
    }
}

impl std::error::Error for ConfigError {}

impl CliConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_args(env::args().skip(1))
    }

    pub fn from_args(args: impl Iterator<Item = String>) -> Result<Self, ConfigError> {
        let mut args = args.peekable();
        let mut origin: Option<String> = None;
        let mut destination: Option<String> = None;
        let mut route_type: Option<RouteType> = None;
        let mut interactive = false;

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                }
                "--interactive" | "-i" => interactive = true,
                "--from" => origin = Some(take_value(&mut args, "--from")?),
                "--to" => destination = Some(take_value(&mut args, "--to")?),
                "--mode" | "-m" => {
                    let raw: String = take_value(&mut args, "--mode")?;
                    route_type = Some(parse_route_mode(&raw)?);
                }
                other if other.starts_with('-') => {
                    return Err(ConfigError::UknownFlag(other.into()));
                }
                _ => return Err(ConfigError::UknownFlag(arg)),
            }
        }

        if interactive {
            return Ok(Self {
                command: Command::Interactive,
            });
        }

        let request = RouteRequest {
            origin: origin.ok_or(ConfigError::MissingRequired("--from"))?,
            destination: destination.ok_or(ConfigError::MissingRequired("--to"))?,
            route_type: route_type.ok_or(ConfigError::MissingRequired("--mode"))?,
        };

        Ok(Self {
            command: Command::Route(request),
        })
    }
}

pub fn read_route_request() -> std::io::Result<Option<RouteRequest>> {
    let origin = read_required_input("Insert origin address (City, Address, Country):")?;
    let destination = read_required_input("Insert destination address (City, Address, Country):")?;

    let Some(route_type) = read_route_mode()? else {
        return Ok(None);
    };

    Ok(Some(RouteRequest {
        origin,
        destination,
        route_type,
    }))
}

fn read_required_input(prompt: &str) -> std::io::Result<String> {
    loop {
        println!("{prompt}");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let value = input.trim();
        if !value.is_empty() {
            return Ok(value.to_owned());
        }
        eprintln!("Input cannot be empty. Please try again.");
    }
}

fn read_route_mode() -> std::io::Result<Option<RouteType>> {
    println!("Choose route mode (driving/cycling/walking) or type 'exit' to quit:");
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let raw = input.trim();

        if raw.eq_ignore_ascii_case("exit") {
            return Ok(None);
        }

        match parse_route_mode(raw) {
            Ok(mode) => return Ok(Some(mode)),
            Err(_) => eprintln!("Invalid mode. Use: driving | cycling | walking (or exit)."),
        }
    }
}

fn parse_route_mode(raw: &str) -> Result<RouteType, ConfigError> {
    match raw.to_ascii_lowercase().as_str() {
        "driving" => Ok(RouteType::Driving),
        "cycling" => Ok(RouteType::Cycling),
        "walking" => Ok(RouteType::Walking),
        _ => Err(ConfigError::InvalidMode(raw.to_string())),
    }
}

fn take_value(
    args: &mut std::iter::Peekable<impl Iterator<Item = String>>,
    flag: &'static str,
) -> Result<String, ConfigError> {
    args.next().ok_or(ConfigError::MissingValue { flag })
}
