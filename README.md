# RustRecon #

RustRecon is a Rust-based recon utility for IP and domain searches using a variety of free, open-source search APIs (aside from Cisco Investigate). This project is still a work-in-progress.

# To Do #
- Filter JSON responses for important information to output
- Email search for applicable search types

# Prerequisites #
- Rustup - Installers -  https://rust-lang.org/tools/install
- Cargo - Install documentation - https://doc.rust-lang.org/cargo/getting-started/installation.html
- API Keys for:
  - Shodan - SHODAN_API
  - Censys - CENSYS_ID
           - CENSYS_SECRET
  - FullHunt - FULLHUNT_API
  - ProjectDiscovery - PROJECTDISCOVERY_API
  - Cisco Investigate - CISCO_INVESTIGATE_API
  - CriminalIP - CRIMINALIP_API
  - Hunter.Io - HUNTERIO_API
  - Netlas - NETLAS_API
  - ZoomEye - ZOOMEYE_API
Note: Each search type API key needs to be set as a local environment variable with the proper name listed above. 
   

