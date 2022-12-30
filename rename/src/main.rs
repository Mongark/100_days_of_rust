use std::fs::{rename};

use clap::Parser;

#[derive(Parser)]
struct Commands {
    from:   String,
    to:     String,
}

fn main() {
    let commands = Commands::parse();
    
    if !commands.from.is_empty() || !commands.to.is_empty() {
        let res = rename(commands.from, commands.to);

        match res {
            Err(_) => println!("ERROR: File not found!"),
            Ok(_) => println!("Name changed successfully"),
        }
    }
}
