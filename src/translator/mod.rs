use std::rc::Rc;

use crate::{
    core::{
        expression::{Expression, ExpressionRef, Operand},
        primitives::Primitive,
    },
    parser::token::{Token, TokenType},
};

pub struct Translator {}

impl Translator {
    pub fn from(tokens: Vec<Token>) -> ExpressionRef {
        let values = Translator::tree(tokens);

        Rc::new(values)
    }

    pub fn tree(tokens: Vec<Token>) -> Expression {
        let mut it = tokens.clone().into_iter();

        let token = match it.next() {
            Some(val) => val,
            None => {
                return Expression::Primitive(Primitive::ToReplace);
            }
        };

        let expression = match token.token_type {
            TokenType::Plus => {
                let right = it.next().unwrap();
                let left = it.next().unwrap();
                Expression::Sum {
                    left: Rc::new(Translator::tree(vec![left])),
                    right: Rc::new(Translator::tree(vec![right])),
                }
            }
            TokenType::Star => {
                let right = it.next().unwrap();
                let left = it.next().unwrap();
                Expression::Product {
                    left: Rc::new(Translator::tree(vec![left])),
                    right: Rc::new(Translator::tree(vec![right])),
                }
            }
            TokenType::Minus => {
                let right = it.next().unwrap();
                let left = it.next().unwrap();
                Expression::Subtract {
                    left: Rc::new(Translator::tree(vec![left])),
                    right: Rc::new(Translator::tree(vec![right])),
                }
            }
            TokenType::Slash => {
                let left = it.next().unwrap();
                let right = it.next().unwrap();
                Expression::Divide {
                    left: Rc::new(Translator::tree(vec![left])),
                    right: Rc::new(Translator::tree(vec![right])),
                }
            }
            TokenType::Print => {
                let mut expression_tokens = Vec::new();
                while let Some(token) = it.next() {
                    if token.token_type == TokenType::EOL {
                        break;
                    }
                    expression_tokens.push(token);
                }
                Expression::Print {
                    expression: Rc::new(Translator::tree(expression_tokens)),
                }
            }
            TokenType::String => {
                let right = token;
                Expression::Primitive(Primitive::String(right.lexeme.clone()))
            }
            TokenType::Number => {
                let right = token;
                Expression::Primitive(Primitive::Integer(right.lexeme.parse::<i32>().unwrap()))
            }
            TokenType::True => Expression::Primitive(Primitive::Boolean(true)),
            TokenType::False => Expression::Primitive(Primitive::Boolean(false)),
            TokenType::LeftParen => {
                let mut expression_tokens = Vec::new();
                while let Some(token) = it.next() {
                    expression_tokens.push(token.clone());
                    if token.token_type == TokenType::RightParen {
                        break;
                    }
                }
                return Translator::tree(expression_tokens);
            }
            TokenType::LeftBrace => {
                let mut expression_tokens = Vec::new();
                while let Some(token) = it.next() {
                    expression_tokens.push(token.clone());
                    if token.token_type == TokenType::RightBrace {
                        break;
                    }
                }
                return Translator::tree(expression_tokens);
            }
            TokenType::Bang => Expression::Unary {
                operand: Operand::Not,
                right: Rc::new(Translator::tree(vec![it.next().unwrap()])),
            },
            TokenType::If => {
                let mut condition_tokens = Vec::new();
                while let Some(token) = it.next() {
                    condition_tokens.push(token.clone());
                    if token.token_type == TokenType::RightParen {
                        break;
                    }
                }
                let mut then_tokens = Vec::new();
                while let Some(token) = it.next() {
                    if token.token_type == TokenType::Else {
                        break;
                    }
                    then_tokens.push(token.clone());
                }
                let mut else_tokens = Vec::new();
                while let Some(token) = it.next() {
                    else_tokens.push(token.clone());
                    if token.token_type == TokenType::RightBrace {
                        break;
                    }
                }

                Expression::If {
                    condition: Rc::new(Translator::tree(condition_tokens)),
                    then_branch: Rc::new(Translator::tree(then_tokens)),
                    else_branch: Rc::new(Translator::tree(else_tokens)),
                }
            }
            TokenType::For => {
                // for <var> in [<from>..<to>] { <body> }

                let variable_name = it.next().unwrap().lexeme;

                it.next().unwrap(); // skip 'in'
                it.next().unwrap(); // skip '['

                let mut from_tokens = Vec::new();
                let mut include_from = true;

                while let Some(token) = it.next() {
                    if token.clone().token_type == TokenType::DotDot {
                        include_from = false;
                        break;
                    }
                    if token.clone().token_type == TokenType::DotDotEqual {
                        break;
                    }
                    from_tokens.push(token.clone());
                }

                let mut to_tokens = Vec::new();
                while let Some(token) = it.next() {
                    if token.clone().token_type == TokenType::RightBracket {
                        break;
                    }
                    to_tokens.push(token.clone());
                }

                let mut body_tokens = Vec::new();
                while let Some(token) = it.next() {
                    body_tokens.push(token.clone());
                    if token.clone().token_type == TokenType::RightBrace {
                        break;
                    }
                }

                let to = if include_from {
                    Rc::new(Translator::tree(to_tokens))
                } else {
                    Rc::new(Expression::Subtract {
                        left: Rc::new(Translator::tree(from_tokens.clone())),
                        right: Rc::new(Expression::Primitive(Primitive::Integer(1))),
                    })
                };

                Expression::For {
                    variable: variable_name,
                    from: Rc::new(Translator::tree(from_tokens.clone())),
                    to,
                    body: Rc::new(Translator::tree(body_tokens.clone())),
                }
            }
            TokenType::Greater => {
                let left = it.next().unwrap();
                let right = it.next().unwrap();
                Expression::Compare {
                    left: Rc::new(Translator::tree(vec![left])),
                    operand: Operand::GreaterThan,
                    right: Rc::new(Translator::tree(vec![right])),
                }
            }
            TokenType::Less => {
                let left = it.next().unwrap();
                let right = it.next().unwrap();
                Expression::Compare {
                    left: Rc::new(Translator::tree(vec![left])),
                    operand: Operand::LessThan,
                    right: Rc::new(Translator::tree(vec![right])),
                }
            }
            TokenType::Let => {
                let mut variable_tokens = Vec::new();

                let name = it.next().unwrap();
                let _ = it.next();

                while let Some(token) = it.next() {
                    variable_tokens.push(token.clone());
                    if token.token_type == TokenType::EOL {
                        break;
                    }
                }

                let value = Rc::new(Translator::tree(variable_tokens.clone()));

                let mut variables = Vec::new();

                variables.push((name.lexeme.clone(), value));

                return Expression::Let {
                    variables,
                    scope: Rc::new(Translator::tree(it.collect())),
                };
            }
            TokenType::Identifier => {
                let right = token;
                Expression::Use {
                    variable: right.lexeme.clone(),
                }
            }
            TokenType::EOL => {
                return Expression::Expression(Rc::new(Translator::tree(it.collect())))
            }
            TokenType::Else => {
                return Expression::Expression(Rc::new(Translator::tree(it.collect())))
            }
            _ => Expression::None {},
        };

        let tokes = it.collect();

        let expression = expression.replace_with(Translator::tree(tokes));

        expression
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_prints() {
        let tokens = vec![
            Token::new(
                TokenType::Print,
                "print".to_string(),
                "print".to_string(),
                1,
            ),
            Token::new(TokenType::String, "john".to_string(), "john".to_string(), 1),
        ];
        let expr = Translator::from(tokens);

        assert_eq!(
            expr,
            Rc::new(Expression::Print {
                expression: Rc::new(Expression::Primitive(Primitive::String("john".to_string())))
            })
        );
    }

    #[test]
    fn it_prints_sum() {
        let tokens = vec![
            Token::new(
                TokenType::Print,
                "print".to_string(),
                "print".to_string(),
                1,
            ),
            Token::new(TokenType::LeftParen, "(".to_string(), "(".to_string(), 1),
            Token::new(TokenType::Plus, "+".to_string(), "+".to_string(), 1),
            Token::new(TokenType::Number, "2".to_string(), "2".to_string(), 1),
            Token::new(TokenType::Number, "1".to_string(), "1".to_string(), 1),
            Token::new(TokenType::RightParen, ")".to_string(), ")".to_string(), 1),
        ];
        let expr = Translator::from(tokens);

        assert_eq!(
            expr,
            Rc::new(Expression::Print {
                expression: Rc::new(Expression::Sum {
                    right: Rc::new(Expression::Primitive(Primitive::Integer(2))),
                    left: Rc::new(Expression::Primitive(Primitive::Integer(1))),
                })
            })
        );
    }

    #[test]
    fn it_makes_it_false() {
        let tokens = vec![
            Token::new(TokenType::Bang, "!".to_string(), "!".to_string(), 1),
            Token::new(TokenType::Number, "2".to_string(), "2".to_string(), 1),
        ];
        let expr = Translator::from(tokens);

        assert_eq!(
            expr,
            Rc::new(Expression::Unary {
                operand: Operand::Not,
                right: Rc::new(Expression::Primitive(Primitive::Integer(2))),
            })
        );
    }

    #[test]
    fn it_makes_it_true() {
        let tokens = vec![
            Token::new(TokenType::Bang, "!".to_string(), "!".to_string(), 1),
            Token::new(TokenType::LeftParen, "(".to_string(), "(".to_string(), 1),
            Token::new(TokenType::Bang, "!".to_string(), "!".to_string(), 1),
            Token::new(TokenType::Number, "1".to_string(), "1".to_string(), 1),
            Token::new(TokenType::RightParen, ")".to_string(), ")".to_string(), 1),
        ];
        let expr = Translator::from(tokens);

        assert_eq!(
            expr,
            Rc::new(Expression::Unary {
                operand: Operand::Not,
                right: Rc::new(Expression::Unary {
                    operand: Operand::Not,
                    right: Rc::new(Expression::Primitive(Primitive::Integer(1))),
                }),
            })
        );
    }

    #[test]
    fn it_can_make_if_statements() {
        let tokens = vec![
            Token::new(TokenType::If, "if".to_string(), "if".to_string(), 1),
            Token::new(TokenType::LeftParen, "(".to_string(), "(".to_string(), 1),
            Token::new(TokenType::Bang, "!".to_string(), "!".to_string(), 1),
            Token::new(TokenType::Number, "1".to_string(), "1".to_string(), 1),
            Token::new(TokenType::RightParen, ")".to_string(), ")".to_string(), 1),
            Token::new(TokenType::LeftBrace, "{".to_string(), "{".to_string(), 1),
            Token::new(
                TokenType::Print,
                "print".to_string(),
                "print".to_string(),
                2,
            ),
            Token::new(TokenType::Number, "1".to_string(), "1".to_string(), 2),
            Token::new(TokenType::RightBrace, "}".to_string(), "}".to_string(), 3),
        ];

        let expr = Translator::from(tokens);
        assert_eq!(
            expr,
            Rc::new(Expression::If {
                condition: Rc::new(Expression::Unary {
                    operand: Operand::Not,
                    right: Rc::new(Expression::Primitive(Primitive::Integer(1))),
                }),
                then_branch: Rc::new(Expression::Print {
                    expression: Rc::new(Expression::Primitive(Primitive::Integer(1)))
                }),
                else_branch: Rc::new(Expression::None {}),
            })
        );
    }

    #[test]
    fn it_can_make_let_statements() {
        let tokens = vec![
            Token::new(TokenType::Let, "let".to_string(), "let".to_string(), 1),
            Token::new(
                TokenType::Identifier,
                "xanax".to_string(),
                "xanax".to_string(),
                1,
            ),
            Token::new(TokenType::Equal, "=".to_string(), "=".to_string(), 1),
            Token::new(TokenType::Number, "1".to_string(), "1".to_string(), 1),
        ];

        let expr = Translator::from(tokens);

        assert_eq!(
            expr,
            Rc::new(Expression::Let {
                variables: vec![(
                    "xanax".to_string(),
                    Rc::new(Expression::Primitive(Primitive::Integer(1)))
                ),],
                scope: Rc::new(Expression::None {}),
            })
        );
    }

    #[test]
    fn it_can_read_variables() {
        let tokens = vec![Token::new(
            TokenType::Identifier,
            "xanax".to_string(),
            "xanax".to_string(),
            1,
        )];

        let expr = Translator::from(tokens);

        assert_eq!(
            expr,
            Rc::new(Expression::Use {
                variable: "xanax".to_string(),
            })
        );
    }
}
