use super::{evaluation::evaluate, expression::ExpressionRef, values::Value};

pub fn interpret(expr: ExpressionRef) -> Value {
    match evaluate(&expr, &None) {
        Ok(value) => {
            value
        }
        Err(err) => panic!("{:?}", err),
    }
}
