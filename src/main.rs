use std::{
    env,
    fs::File,
    io::{stdin, BufReader, Read, Result},
};

use crate::interpreter::interpret;

mod interpreter;
mod lexer;
mod parser;

fn cli() -> Result<()> {
    let mut buffer = String::new();

    loop {
        print!("> ");
        match stdin().read_line(&mut buffer)? {
            0 => {
                println!("");
                break;
            }
            _ => {
                interpret(buffer.as_bytes())?;
            }
        }
    }

    Ok(())
}

fn run_file(path: &str) -> Result<()> {
    let fd = File::open(path)?;
    let mut buffer = Vec::with_capacity(fd.metadata()?.len() as usize);
    let mut reader = BufReader::new(fd);

    reader.read_to_end(&mut buffer)?;

    interpreter::interpret(&buffer)
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.as_slice() {
        [] => {
            cli().unwrap();
        }
        [filepath] => {
            run_file(filepath).unwrap();
        }
        _ => {
            println!("Usage: notjs [path]");
        }
    }
}
