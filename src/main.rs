use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

struct Args {
    from: String,
    to: String,
    directory: String,
}

fn parse_args() -> Result<Args, String> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    if args.len() != 4 {
        return Err(String::from("Usage: program <from> <to> <directory>"));
    }
    Ok(Args {
        from: args[1].clone(),
        to: args[2].clone(),
        directory: args[3].clone(),
    })
}

fn replace_in_file(path: &Path, from: &str, to: &str) -> std::io::Result<()> {
    let mut content = String::new();
    let mut file = fs::File::open(path)?;
    file.read_to_string(&mut content)?;

    let new_content = content.replace(from, to);

    let mut file = fs::File::create(path)?;
    file.write_all(new_content.as_bytes())?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_args()?;

    for entry in fs::read_dir(args.directory)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            replace_in_file(&path, &args.from, &args.to)?;
            println!("Processed: {:?}", path);
        }
    }

    Ok(())
}