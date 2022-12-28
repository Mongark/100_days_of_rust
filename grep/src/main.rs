use std::fs::read_to_string;
use clap::{Parser};

fn grep_from_file(filename: &String, value: &String) -> String {
    let file_data: String = read_to_string(filename).unwrap();

    if !file_data.contains(value) {
        return String::from("String not found!");
    }

    let file_str: &str = &file_data.as_str();
    let splited = file_str.split('\n');
    let res = splited.filter(|line| line.contains(value)).collect();

    res
}

#[derive(Parser)]
struct Commands {
    #[arg(short, long)]
    file: String,

    #[arg(short, long)]
    value: String,
}

fn main() {
    let args = Commands::parse();

    let res = grep_from_file(&args.file, &args.value);

    println!("{}", res);
}
