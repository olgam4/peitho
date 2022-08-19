use std::rc::Rc;

use super::primitives::Primitive;

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Equals,
    LessThan,
    GreaterThan,
    Not,
    Negate,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Primitive(Primitive),
    Sum {
        left: ExpressionRef,
        right: ExpressionRef,
    },
    Product {
        left: ExpressionRef,
        right: ExpressionRef,
    },
    Divide {
        left: ExpressionRef,
        right: ExpressionRef,
    },
    If {
        condition: ExpressionRef,
        then_branch: ExpressionRef,
        else_branch: ExpressionRef,
    },
    Compare {
        left: ExpressionRef,
        operand: Operand,
        right: ExpressionRef,
    },
    Let {
        variables: Vec<(String, ExpressionRef)>,
        scope: ExpressionRef,
    },
    Use {
        variable: String,
    },
    Unary {
        operand: Operand,
        right: ExpressionRef,
    },
    Assign {
        variable: String,
        value: ExpressionRef,
    },
    For {
        variable: String,
        from: ExpressionRef,
        to: ExpressionRef,
        body: ExpressionRef,
    },
    Print {
        expression: ExpressionRef,
    },
    Chain {
        left: ExpressionRef,
        right: ExpressionRef,
    },
    DeriveState {
        expression: ExpressionRef,
    },
    None {},
}
pub type ExpressionRef = Rc<Expression>;
