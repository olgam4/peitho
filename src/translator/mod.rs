use std::rc::Rc;

use crate::{
    core::{
        expression::{Expression, ExpressionRef},
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
            _ => Expression::None {},
        };
        Rc::new(expr)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::token::TokenType;

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
}
