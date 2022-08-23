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
    None,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    InvalidOperand(Operand),
    InvalidExpression(Expression),
    InvalidValues(String, Vec<Value>),
    UndefinedVariable(String),
}
