# RustRecon #

# RustRecon is a Rust-based recon utility for IP and domain searches using a variety of free, open-source search APIs (aside from Cisco Investigate). This project is still a work-in-progress.

- Note: Although the API keys for each source are free to use, some functionality may only be accessible with paid subscriptions. #
- Also, several of the free versions of these sources have a limited number of requests per month...so either pay for the service or don't overuse your search amounts. #

# To Do #
- Filter JSON responses for output
- Email search for applicable search types

# Prerequisites #
- Python3
- Rustup - Installers -  https://rust-lang.org/tools/install
- Cargo - Install documentation - https://doc.rust-lang.org/cargo/getting-started/installation.html
- API Keys for:
  - Shodan - SHODAN_API
  - Censys - CENSYS_ID
           - CENSYS_SECRET
  - FullHunt - FULLHUNT_API
  - ProjectDiscovery - PROJECTDISCOVERY_API
  - CriminalIP - CRIMINALIP_API
  - Hunter.Io - HUNTERIO_API
  - Netlas - NETLAS_API
  - ZoomEye - ZOOMEYE_API
 
  - Optional Prerequisite
      * chaos - https://github.com/projectdiscovery/chaos-client
   (If you don't have the chaos client installed, the projectdiscovery search will query the API instead)

# Installation #
Note: Before compiling, set each search type API key needs to be set as a local environment variable with the exact name listed above.

 - git clone https://github.com/xBurningGiraffe/rustrecon.git
 - cd rustrecon
 - cargo build

# Usage #

USAGE:
    rust_recon.exe [OPTIONS] --target <TARGET>

OPTIONS:
    -a, --all                          Run all applicable search types on the target
    -h, --help                         Print help information
    -o, --output <FILE>                Output the results to a file
        --search_type <SEARCH_TYPE>    The type(s) of search, separated by commas [possible values:
                                       shodan, censys, fullhunt, projectdiscovery, criminalip,
                                       hunterio, netlas, zoomeye, internetdb]
        --target <TARGET>              The target IP address or domain


# Examples #

- rust_recon -a --target google.com

- rust_recon --search_type shodan --target google.com

- rust_recon --search_type shodan,censys --target 1.1.1.1 -o output.txt

