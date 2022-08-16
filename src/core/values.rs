use std::collections::HashMap;


use super::expression::{ExpressionRef, Operand, Expression};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Integer(i32),
    Float(f64),
    Boolean(bool),
    State(Option<HashMap<String, ExpressionRef>>),
    Unit,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    InvalidOperand(Operand),
    InvalidExpression(Expression),
    InvalidValues(Vec<Value>),
    UndefinedVariable(String),
}
