use clap::Parser;

/// A CLI application that takes origin, replace, and path arguments
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The origin string
    #[arg(long, default_value = "default_origin")]
    origin: String,

    /// The replace string
    #[arg(long, default_value = "default_replace")]
    replace: String,

    /// The path to the file or directory
    #[arg(long, default_value = "./default_path.txt")]
    path: String,
}

fn main() {
    let args = Args::parse();

    println!("Origin: {}", args.origin);
    println!("Replace: {}", args.replace);
    println!("Path: {}", args.path);

    if args.origin == "default_origin" && args.replace == "default_replace" && args.path == "./default_path.txt" {
        println!("Running with default values. Use --help for more information on available options.");
    } else {
        println!("Running with provided values.");
    }
}
