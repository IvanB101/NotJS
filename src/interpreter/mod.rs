// use lazy_static::lazy_static;
// use std::sync::RwLock;

use crate::error::generic::GenericResult;
use crate::parser;

use self::environment::Environment;

// lazy_static! {
//     static ref ENVIRONMENT: RwLock<Environment> = RwLock::new(Environment::new());
// }
pub mod environment;
mod evaluate;
mod execute;

pub struct Interpreter {
    pub environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, source: &[u8]) -> GenericResult<()> {
        let statements = parser::parse(source)?;

        // println!("{:?}", statements);
        for statement in statements {
            statement.execute(&mut self.environment)?;
        }

        // println!("ENV: {:?}", self.environment);

        Ok(())
    }
}

pub fn interpret(source: &[u8]) -> GenericResult<()> {
    let mut interpreter = Interpreter::new();

    interpreter.interpret(source)
}

#[cfg(test)]
mod tests {
    use crate::interpret;

    #[test]
    fn test_interpret_string_index() {
        let source = br#"
            let str = "hello";
            let char = str[1];
        "#;
        interpret(source).unwrap();
    }

    #[test]
    fn test_interpret_string_length() {
        let source = br#"
            let str = "hello";
            let length = str.length;
        "#;
        interpret(source).unwrap();
    }
}
