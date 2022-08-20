use std::collections::HashMap;

use super::{evaluation::evaluate, expression::ExpressionRef, values::Value};

pub fn interpret(expr: ExpressionRef) -> Value {
    println!("Will try to evaluate...");
    println!("{:?}", expr);
    match evaluate(&expr, &None) {
        Ok(value) => value,
        Err(err) => {
            println!("{:?}", err);
            Value::None
        }
    }
}

pub fn interpret_with_state(expr: ExpressionRef, state: Option<HashMap<String, ExpressionRef>>) -> Value {
    match evaluate(&expr, &state) {
        Ok(value) => value,
        Err(err) => {
            println!("{:?}", err);
            Value::None
        }
    }
}
