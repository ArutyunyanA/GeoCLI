pub fn print_help() {
    println!(
        "\
🗺️  GeoApp — Route CLI\n\
\n\
USAGE:\n\
  geoapp --from <ADDRESS> --to <ADDRESS> --mode <driving|cycling|walking>\n\
  geoapp --interactive\n\
\n\
EXAMPLES:\n\
  geoapp --from \"Budapest, Alagút u. 4\" --to \"Vienna, Stephansplatz\" --mode driving\n\
  geoapp --interactive\n\
\n\
OPTIONS:\n\
  --from <ADDRESS>         Origin address\n\
  --to <ADDRESS>           Destination address\n\
  --mode, -m <MODE>        Route mode: driving | cycling | walking\n\
  --interactive, -i        Start interactive mode\n\
  --help, -h               Show this help message\n\
\n\
INTERACTIVE MODE TIPS:\n\
  • Enter full addresses for better geocoding accuracy\n\
  • Type 'exit' when asked for mode to quit"
    );
}
