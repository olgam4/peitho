use std::collections::HashMap;

use super::{evaluation::evaluate, expression::ExpressionRef, values::Value};

pub fn interpret(expr: ExpressionRef) -> Value {
    match evaluate(&expr, &None) {
        Ok(value) => {
            value
        }
        Err(err) => panic!("{:?}", err),
    }
}

pub fn interpret_with_state(expr: ExpressionRef, state: HashMap<String, ExpressionRef>) -> Value {
    match evaluate(&expr, &Some(state)) {
        Ok(value) => {
            value
        }
        Err(err) => panic!("{:?}", err),
    }
}
