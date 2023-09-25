pub fn print_help() {
    println!("Usage: RustRecon [command]");
    println!("Options:");
    println!("  -t                Specify the target (IP, domain)");
    println!("  -l                List of targets (IPs or domains");
    println!("  -o                Output the results to a file");
    println!("  -all              Run all applicable search types on target");
    println!("  help              Print this screen");
    println!();
    println!("Available search types:");
    println!("  shodan            Domain and IP search using Shodan");
    println!("  investigate       Domain security check with Cisco Investigate");
    println!("  censys            IP search using Censys");
    println!("  fullhunt          Domain search using FullHunt");
    println!("  projectdiscovery  Domain search using ProjectDiscovery");
    println!("  criminalip        IP search using CriminalIP");
    println!("  hunterio          Domain search using HunterIO");
    println!("  netlas            IP or domain search using Netlas");
    println!("  zoomeye           IP search using ZoomEye");
    println!("  internetdb        Vulnerability check for IP using internetdb");
}