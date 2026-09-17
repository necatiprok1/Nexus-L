use crate::ast::{BinaryOperator, Expr, Program, Statement};
use std::collections::HashMap;

pub struct Interpreter {
    variables: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn execute(&mut self, program: &Program) {
        for statement in &program.statements {
            self.execute_statement(statement);
        }
    }

    fn execute_statement(&mut self, statement: &Statement) {
        match statement {
            Statement::Print(expression) => {
                let value = self.evaluate(expression);
                println!("{}", value);
            }

            Statement::Let { name, value } => {
                let evaluated = self.evaluate_value(value);
                self.variables.insert(name.clone(), evaluated);
            }
        }
    }

    fn evaluate(&self, expression: &Expr) -> String {
        match self.evaluate_value(expression) {
            Value::String(value) => value,
            Value::Number(value) => format_number(value),
        }
    }

    fn evaluate_number(&self, expression: &Expr) -> f64 {
        match expression {
            Expr::Number(value) => *value,

            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.evaluate_number(left);
                let right = self.evaluate_number(right);

                match operator {
                    BinaryOperator::Add => left + right,
                    BinaryOperator::Subtract => left - right,
                    BinaryOperator::Multiply => left * right,
                    BinaryOperator::Divide => left / right,
                }
            }

            Expr::String(_) => {
                panic!("Expected number, found string")
            }

            Expr::Identifier(name) => match self.variables.get(name) {
                Some(Value::Number(value)) => *value,

                Some(Value::String(_)) => {
                    panic!("Expected number, found string")
                }

                None => {
                    panic!("Undefined variable: {}", name)
                }
            },
        }
    }
}

fn evaluate_value(&self, expression: &Expr) -> Value {
    match expression {
        Expr::String(value) => Value::String(value.clone()),

        Expr::Number(value) => Value::Number(*value),

        Expr::Identifier(name) => self
            .variables
            .get(name)
            .cloned()
            .unwrap_or_else(|| panic!("Undefined variable: {}", name)),

        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let left = self.evaluate_number(left);
            let right = self.evaluate_number(right);

            let result = match operator {
                BinaryOperator::Add => left + right,
                BinaryOperator::Subtract => left - right,
                BinaryOperator::Multiply => left * right,
                BinaryOperator::Divide => left / right,
            };

            Value::Number(result)
        }
    }
}

fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else {
        value.to_string()
    }
}
