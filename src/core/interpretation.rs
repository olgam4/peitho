use super::{evaluation::evaluate, expression::ExpressionRef, values::Value};

pub fn interpret(expr: ExpressionRef) -> Value {
    println!("will interpret...");
    println!("{:?}", expr);
    match evaluate(&expr, &None) {
        Ok(value) => {
            println!("it evaluated to {:?}", value);
            value
        }
        Err(err) => panic!("{:?}", err),
    }
}
