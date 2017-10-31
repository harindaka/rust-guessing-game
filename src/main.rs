#[macro_use]
extern crate serde_derive;

use std::io::{self, Write};
mod guessing_game;
mod configuration;
mod config;

fn main() {

    loop {
        println!("Enter your selection:");
        println!("0. Exit");
        println!("1. Guessing Game");
        println!("2. Read File and JSON serialization");
        
        print!(">> ");
        io::stdout().flush().unwrap();
        
        let mut selected_option = String::new();

        io::stdin().read_line(&mut selected_option)
            .expect("Failed to read line");

        let selected_option: i32 = match selected_option.trim().parse(){
            Ok(num) => num,
            Err(_) => -1,
        };

        println!("");
        
        match selected_option {
            0 => break,
            1 => guessing_game::run_example(),
            2 => configuration::build_config(),
            3 => println!("Not implemented"),
            _ => continue,
        }

        println!("");
    }
}