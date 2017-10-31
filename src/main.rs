
use std::io::{self, Write};
mod guessing_game;

fn main() {

    loop {
        println!("Select your example to run");
        println!("0. Exit");
        println!("1. Guessing Game");
        println!("2. Read File");
        println!("3. JSON Serialization and Merge");
        
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
            2 => println!("Not implemented"),
            3 => println!("Not implemented"),
            _ => continue,
        }

        println!("");
    }
}