use clap::Parser;
use std::{fs, path};


#[derive(Parser)]
struct Cli {
    pattern: String,
    path: path::PathBuf
}

#[derive(Debug)]
struct CustomeError(String);

fn main() -> Result<(), CustomeError> {
    let args = Cli::parse();
    let pattern = args.pattern;
    let path = path::PathBuf::from(args.path);
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => return Err(CustomeError(format!("File NotFound {}",err)))
    };
    for line in content.lines(){
        if line.contains(&pattern){
            println!("{}", line);
        }
    }
    Ok(())
}