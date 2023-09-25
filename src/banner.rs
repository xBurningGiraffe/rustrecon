use colored::*;
use term_size;

pub fn display_banner() {
    let banner_text = r#"

    ███████████                       █████    ███████████                                        
    ░░███░░░░░███                     ░░███    ░░███░░░░░███                                       
    ░███    ░███  █████ ████  █████  ███████   ░███    ░███   ██████   ██████   ██████  ████████  
    ░██████████  ░░███ ░███  ███░░  ░░░███░    ░██████████   ███░░███ ███░░███ ███░░███░░███░░███ 
    ░███░░░░░███  ░███ ░███ ░░█████   ░███     ░███░░░░░███ ░███████ ░███ ░░░ ░███ ░███ ░███ ░███ 
    ░███    ░███  ░███ ░███  ░░░░███  ░███ ███ ░███    ░███ ░███░░░  ░███  ███░███ ░███ ░███ ░███ 
    █████   █████ ░░████████ ██████   ░░█████  █████   █████░░██████ ░░██████ ░░██████  ████ █████
    ░░░░░   ░░░░░   ░░░░░░░░ ░░░░░░     ░░░░░  ░░░░░   ░░░░░  ░░░░░░   ░░░░░░   ░░░░░░  ░░░░ ░░░░░

    Made by xBurningGiraffe

    "#;

    if let Some((w, _)) = term_size::dimensions() {
        let banner_lines: Vec<&str> = banner_text.lines().collect();

        for line in banner_lines {
            let padding = (w as usize - line.chars().count()) / 2;
            let padded_line: String = " ".repeat(padding) + line;
            println!("{}", padded_line.green());
        }
    } else {
        // Now banner_text is accessible here as well
        println!("{}", banner_text.green());
    }
}