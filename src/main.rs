use figlet_rs::FIGfont;
use std::io;

fn main() {
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let font = FIGfont::standard().unwrap();
    let figure = font.convert(&*user_input.trim().to_uppercase());

    println!("{}",  figure.unwrap())

}
