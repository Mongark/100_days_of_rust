use clap::Parser;

fn copy(from: String, to: String) -> Result<u64, std::io::Error> {
    std::fs::copy(from, to)
}

#[derive(Parser)]
struct Commands {
    from: String,
    to: String,
}

fn main() {
    let args = Commands::parse();

    copy(args.from, args.to).expect("No such file or directory");
}
