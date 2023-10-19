use std::{
    env,
    fs::File,
    io::{stdin, stdout, BufReader, Read, Result, Write},
};

use crate::interpreter::interpret;

mod interpreter;
mod lexer;
mod parser;

fn cli() -> Result<()> {
    let mut buffer = String::new();

    loop {
        print!("> ");
        stdout().flush()?;
        match stdin().read_line(&mut buffer)? {
            0 => {
                println!("");
                break;
            }
            _ => {
                interpret(buffer.as_bytes())?;
                buffer.clear();
                println!("");
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
            let source = b"8 / 2 - 1 == 1 + 2 * 3;";

            interpreter::interpret(source).unwrap();

            println!("\nEjecucion de CLI: ");

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
