use crate::{
    common::{
        statements::{
            BlockStatement, ExpressionStatement, FunctionStatement, IfStatement, PrintStatement,
            ReturnStatement, Statement, VariableDeclaration, WhileStatement,
        },
        value::Value,
    },
    error::runtime::RuntimeResult,
};

use super::environment::Environment;

impl Statement for BlockStatement {
    fn execute(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        let mut result = Value::Null;

        environment.push();

        for statement in &self.statements {
            result = statement.execute(environment)?;
        }

        environment.pop();

        Ok(result)
    }
}

impl Statement for VariableDeclaration {
    fn execute(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        match self.initializer {
            Some(ref initializer) => {
                let value = initializer.evaluate(environment)?;
                environment.define(self.identifier.clone(), Some(value), self.mutable);
                Ok(Value::Null)
            }
            None => {
                environment.define(self.identifier.clone(), None, self.mutable);
                Ok(Value::Null)
            }
        }
    }
}

impl Statement for FunctionStatement {
    fn execute(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        environment.define(
            self.name.clone(),
            Some(Value::Function(Box::new(self.clone()))),
            false,
        );
        Ok(Value::Null)
    }
}

impl Statement for ExpressionStatement {
    fn execute(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        self.expression.evaluate(environment)
    }
}

impl Statement for PrintStatement {
    fn execute(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        let value = self.expression.evaluate(environment)?;

        if self.new_line {
            println!("{}", value);
        } else {
            print!("{}", value);
        }

        Ok(value)
    }
}

impl Statement for IfStatement {
    fn execute(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        let condition = self.condition.evaluate(environment)?;

        if condition.is_truthy() {
            self.then_branch.execute(environment)
        } else if let Some(ref else_branch) = self.else_branch {
            else_branch.execute(environment)
        } else {
            Ok(Value::Null)
        }
    }
}

impl Statement for WhileStatement {
    fn execute(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        let mut result = Value::Null;

        while self.condition.evaluate(environment)?.is_truthy() {
            result = self.body.execute(environment)?;
        }

        Ok(result)
    }
}

impl Statement for ReturnStatement {
    fn execute(&self, environment: &mut Environment) -> RuntimeResult<Value> {
        if let Some(ref value) = self.value {
            value.evaluate(environment)
        } else {
            Ok(Value::Null)
        }
    }
}
