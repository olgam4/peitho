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
    Expression(ExpressionRef),
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
    Subtract {
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

impl Expression {
    pub fn replace_with(&self, expression: Expression) -> Expression {
        let mut old_expr = self.clone();
        let final_ = Expression::replace_with_inner(&mut old_expr, expression.clone());
        final_
    }

    pub fn replace_with_inner(old_expr: &mut Expression, expression: Expression) -> Expression {
        match old_expr {
            Expression::Primitive(primitive) => {
                match primitive {
                    Primitive::ToReplace => {
                        expression.clone()
                    }
                    _ => old_expr.clone(),
                }
            }
            Expression::Expression(expr) => {
                Expression::Expression(Rc::new(expr.replace_with(expression.clone())))
            }
            Expression::Sum { left, right } => Expression::Sum {
                left: Rc::new(left.replace_with(expression.clone())),
                right: Rc::new(right.replace_with(expression.clone())),
            },
            Expression::Product { left, right } => Expression::Product {
                left: Rc::new(left.replace_with(expression.clone())),
                right: Rc::new(right.replace_with(expression.clone())),
            },
            Expression::Divide { left, right } => Expression::Divide {
                left: Rc::new(left.replace_with(expression.clone())),
                right: Rc::new(right.replace_with(expression.clone())),
            },
            Expression::Subtract { left, right } => Expression::Subtract {
                left: Rc::new(left.replace_with(expression.clone())),
                right: Rc::new(right.replace_with(expression.clone())),
            },
            Expression::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let else_branch = Rc::new(else_branch.replace_with(expression.clone()));
                Expression::If {
                    condition: Rc::new(condition.replace_with(expression.clone())),
                    then_branch: Rc::new(then_branch.replace_with(expression.clone())),
                    else_branch,
                }
            }
            Expression::Compare {
                left,
                operand,
                right,
            } => Expression::Compare {
                left: Rc::new(left.replace_with(expression.clone())),
                operand: operand.clone(),
                right: Rc::new(right.replace_with(expression.clone())),
            },
            Expression::Let { variables, scope } => {
                let variables = variables
                    .into_iter()
                    .map(|(name, expr)| {
                        (name.clone(), Rc::new(expr.replace_with(expression.clone())))
                    })
                    .collect();
                Expression::Let {
                    variables,
                    scope: Rc::new(scope.replace_with(expression.clone())),
                }
            }
            Expression::Use { variable } => Expression::Use {
                variable: variable.clone(),
            },
            Expression::Unary { operand, right } => Expression::Unary {
                operand: operand.clone(),
                right: Rc::new(right.replace_with(expression.clone())),
            },
            Expression::Assign { variable, value } => Expression::Assign {
                variable: variable.clone(),
                value: Rc::new(value.replace_with(expression.clone())),
            },
            Expression::For {
                variable,
                from,
                to,
                body,
            } => Expression::For {
                variable: variable.clone(),
                from: Rc::new(from.replace_with(expression.clone())),
                to: Rc::new(to.replace_with(expression.clone())),
                body: Rc::new(body.replace_with(expression.clone())),
            },
            Expression::Print { expression: expr } => Expression::Print {
                expression: Rc::new(expr.replace_with(expression.clone())),
            },
            Expression::Chain { left, right } => {
                left.replace_with(expression.clone());
                right.replace_with(expression.clone());
                Expression::Chain {
                    left: left.clone(),
                    right: right.clone(),
                }
            }
            Expression::DeriveState { expression: expr } => Expression::DeriveState {
                expression: Rc::new(expr.replace_with(expression.clone())),
            },
            Expression::None {} => Expression::None {},
        }
    }
}
