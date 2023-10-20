use std::{
    env,
    fs::File,
    io::{stdin, stdout, BufReader, Read, Result, Write},
};

use crate::interpreter::interpret;

mod common;
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

fn debug_file(path: &str) -> Result<()> {
    println!("DEBUG MODE");
    let fd = File::open(path)?;
    let mut buffer = Vec::with_capacity(fd.metadata()?.len() as usize);
    let mut reader = BufReader::new(fd);

    reader.read_to_end(&mut buffer)?;

    let expr = parser::parse(&buffer)?;

    print!("{:#?} => ", expr);

    interpret(&buffer)?;

    Ok(())
}

fn debug_cli() -> Result<()> {
    println!("DEBUG MODE");
    let mut buffer = String::new();

    loop {
        print!("~> ");
        stdout().flush()?;
        match stdin().read_line(&mut buffer)? {
            0 => {
                println!("");
                break;
            }
            _ => {
                let expr = parser::parse(buffer.as_bytes())?;
                print!("{:#?}", expr);
                print!(" => ");
                interpret(buffer.as_bytes())?;
                buffer.clear();
                println!("");
            }
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.as_slice() {
        [] => {
            println!("\nEjecucion de CLI: ");
            cli().unwrap();
        }
        [filepath] => {
            if filepath.ends_with(".notjs") {
                run_file(filepath).unwrap();
            } else {
                println!("File must have .notjs extension");
                println!("Usage: notjs [path] [-dev]");
            }
        }
        [filepath, arg2] => {
            if filepath.ends_with(".notjs") && arg2 == "-dev" {
                debug_file(filepath).unwrap();
            } else if arg2 == "-dev" {
                debug_cli().unwrap();
            } else {
                println!("Usage: notjs [path] [-dev]");
            }
        }
        _ => {
            println!("Usage: notjs [path] [-dev]");
        }
    }
}
