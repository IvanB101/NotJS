use std::io::Result;

use crate::{parser::{self, Expr}, lexer::Value};

pub fn interpret(source: &[u8]) -> Result<()> {
    let expr = parser::parse(source)?;

    print!("{:?}", expr);

    Ok(())
}
