# GeoCLI

A Rust-based command-line application for working with geolocation APIs and endpoints to fetch valid and useful geographic data.

## Overview

GeoCLI is a CLI tool designed for refactoring and testing API endpoints. It provides a robust foundation for consuming geolocation services and processing geographic data with reliability and performance in mind.

## Requirements

- **Rust** 1.70 or later
- **Cargo** (comes with Rust)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/ArutyunyanA/GeoCLI.git
cd GeoCLI
```

2. Build the project:
```bash
cargo build --release
```

Run the application:
```bash
cargo run --release
```

Quick Start

Basic Usage
```bash
# Run with specific features
cargo run -- --help
🗺️  GeoApp — Route CLI

USAGE:
geoapp --from <ADDRESS> --to <ADDRESS> --mode <driving|cycling|walking>
geoapp --interactive

EXAMPLES:
geoapp --from "Budapest, Alagút u. 4" --to "Vienna, Stephansplatz" --mode driving
geoapp --interactive

OPTIONS:
--from <ADDRESS>         Origin address
--to <ADDRESS>           Destination address
--mode, -m <MODE>        Route mode: driving | cycling | walking
--interactive, -i        Start interactive mode
--help, -h               Show this help message

INTERACTIVE MODE TIPS:
• Enter full addresses for better geocoding accuracy
• Type 'exit' when asked for mode to quit

cargo run -- --from "Kajakaška c. 52, 1211 Ljubljana - Šmartno, Slovenia" --to "Bravničarjeva ulica 16, 1000 Ljubljana, Slovenia" --mode walking

output:
Calculated routes for mode Walking:
Route # 1: 5.43 km | 01h 04m 18s
```

Environment Setup

Create a .env file in the project root to configure your API endpoints:
```env
API_KEY=your_api_key_here
BASE_URL=https://api.example.com
```

The application uses dotenvy for environment variable management.
Dependencies

    reqwest (0.11) - Async HTTP client with JSON support
    tokio (1.x) - Async runtime with full feature set
    serde (1.0) - Serialization/deserialization framework
    serde_json (1.0) - JSON handling
    urlencoding (2.1) - URL encoding utilities
    dotenvy (0.15) - Environment variable loading
    thiserror (1.0) - Error handling and custom error types

Project Structure

```code
.
├── Cargo.lock
├── Cargo.toml
└── src
    ├── api.rs
    ├── app.rs
    ├── config.rs
    ├── main.rs
    ├── services.rs
    └── ui.rs

```


Features

    ✅ Async HTTP requests handling
    ✅ JSON serialization/deserialization
    ✅ Environment-based configuration
    ✅ Custom error handling
    ✅ URL encoding support

Roadmap

    Add support for multiple API providers
    Implement caching mechanism
    Add geographic data filtering
    Create comprehensive CLI options
    Add unit and integration tests
    Support for batch requests

Contributing

Contributions are welcome! Feel free to open issues and submit pull requests.
License

This project is open source and available under the MIT License.
Author

Arutyunyan Artem
arutyunyan_av@icloud.com






