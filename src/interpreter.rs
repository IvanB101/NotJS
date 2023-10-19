use std::io::Result;

use crate::parser::{self, print_ast_from_expr};

pub fn interpret(source: &[u8]) -> Result<()> {
    let expr = parser::parse(source)?;

    // println!("{:?}", expr);
    print_ast_from_expr(expr);

    Ok(())
}
