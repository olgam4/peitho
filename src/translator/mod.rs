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
        while let Some(token) = it.next() {
            expr = match token.token_type {
                TokenType::Plus => {
                    let right = it.next().unwrap();
                    let left = it.next().unwrap();
                    println!("{} + {}", left, right);
                    Expression::Sum {
                        left: Translator::from(vec![left]),
                        right: Translator::from(vec![right]),
                    }
                }
                TokenType::Print => {
                    let right = it.next().unwrap();
                    Expression::Print {
                        expression: Translator::from(vec![right]),
                    }
                }
                TokenType::String => {
                    let right = token;
                    Expression::Primitive(Primitive::String(right.lexeme.clone()))
                }
                _ => {
                    break;
                }
            };
        };
        Rc::new(expr)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::token::TokenType;

    use super::*;

    #[test]
    fn it_works() {
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
}
