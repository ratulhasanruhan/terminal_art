use figlet_rs::FIGfont;
use std::io;

fn main() {
    let mut command_input = String::new();
    io::stdin().read_line(&mut command_input).expect("Failed to read line");
    let command_input = command_input.trim();

    if let Some(rest) = command_input.strip_prefix("art ") {
        let font = FIGfont::standard().unwrap();
        let figure = font.convert(&rest.trim().to_uppercase());
        println!("{}", figure.unwrap());
    } else {
        println!("Unknown command. Use 'art <text>' to print ASCII art.");
    }
}
