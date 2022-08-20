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
        let mut it = tokens.clone().into_iter();
        let mut expr = Expression::None {};
        let token = match it.next() {
            Some(token) => token,
            None => return Rc::new(expr),
        };
        expr = match token.token_type {
            TokenType::Plus => {
                let right = it.next().unwrap();
                let left = it.next().unwrap();
                Expression::Sum {
                    left: Translator::from(vec![left]),
                    right: Translator::from(vec![right]),
                }
            }
            TokenType::Star => {
                let right = it.next().unwrap();
                let left = it.next().unwrap();
                Expression::Product {
                    left: Translator::from(vec![left]),
                    right: Translator::from(vec![right]),
                }
            }
            TokenType::Minus => {
                let right = it.next().unwrap();
                let left = it.next().unwrap();
                Expression::Subtract {
                    left: Translator::from(vec![left]),
                    right: Translator::from(vec![right]),
                }
            }
            TokenType::Slash => {
                let left = it.next().unwrap();
                let right = it.next().unwrap();
                Expression::Divide {
                    left: Translator::from(vec![left]),
                    right: Translator::from(vec![right]),
                }
            }
            TokenType::Print => Expression::Print {
                expression: Translator::from(it.collect()),
            },
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
                let tokens = it
                    .take_while(|t| t.token_type != TokenType::RightParen)
                    .collect();
                return Translator::from(tokens);
            }
            TokenType::LeftBrace => {
                let tokens = it
                    .take_while(|t| t.token_type != TokenType::RightBrace)
                    .collect();
                return Translator::from(tokens);
            }
            TokenType::Bang => Expression::Unary {
                operand: Operand::Not,
                right: Translator::from(it.collect()),
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
                    condition: Translator::from(condition_tokens),
                    then_branch: Translator::from(then_tokens),
                    else_branch: Translator::from(else_tokens),
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

                let to = if include_from {
                    Translator::from(to_tokens)
                } else {
                    Rc::new(Expression::Subtract {
                        left: Translator::from(to_tokens),
                        right: Rc::new(Expression::Primitive(Primitive::Integer(1))),
                    })
                };

                Expression::For {
                    variable: variable_name,
                    from: Translator::from(from_tokens),
                    to,
                    body: Translator::from(it.collect()),
                }
            }
            TokenType::Greater => {
                let left = it.next().unwrap();
                let right = it.next().unwrap();
                Expression::Compare {
                    left: Translator::from(vec![left]),
                    operand: Operand::GreaterThan,
                    right: Translator::from(vec![right]),
                }
            }
            TokenType::Less => {
                let left = it.next().unwrap();
                let right = it.next().unwrap();
                Expression::Compare {
                    left: Translator::from(vec![left]),
                    operand: Operand::LessThan,
                    right: Translator::from(vec![right]),
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

                let value = Translator::from(variable_tokens);

                let mut variables = Vec::new();

                variables.push((name.lexeme.clone(), value));
                Expression::Let {
                    variables,
                    scope: Translator::from(it.collect()),
                }
            }
            TokenType::Identifier => {
                let right = token;
                Expression::Use {
                    variable: right.lexeme.clone(),
                }
            }
            TokenType::EOL => Expression::Chain {
                left: Translator::from(it.collect()),
                right: Rc::new(Expression::None {}),
            },
            _ => Expression::None {},
        };

        Rc::new(expr)
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
