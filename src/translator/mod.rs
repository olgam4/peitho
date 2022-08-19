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
        let mut it = tokens.into_iter();
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
                    then_tokens.push(token.clone());
                    if token.token_type == TokenType::Else {
                        break;
                    }
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
            },
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
}
