use std::{collections::HashMap, rc::Rc};

use super::{
    expression::{Expression, ExpressionRef, Operand},
    primitives::Primitive,
    values::{Error, Value},
};

pub fn evaluate(
    expr: &Expression,
    state: &Option<HashMap<String, ExpressionRef>>,
) -> Result<Value, Error> {
    match expr {
        Expression::None {} => Ok(Value::None),
        Expression::Sum { left, right } => {
            let left_value = evaluate(&left, state)?;
            let right_value = evaluate(&right, state)?;
            match (left_value.clone(), right_value.clone()) {
                (Value::Integer(left), Value::Integer(right)) => Ok(Value::Integer(left + right)),
                _ => Err(Error::InvalidValues(vec![left_value, right_value])),
            }
        }
        Expression::Product { left, right } => {
            let left_value = evaluate(&left, state)?;
            let right_value = evaluate(&right, state)?;
            match (left_value.clone(), right_value.clone()) {
                (Value::Integer(left), Value::Integer(right)) => Ok(Value::Integer(left * right)),
                _ => Err(Error::InvalidValues(vec![left_value, right_value])),
            }
        }
        Expression::Divide { left, right } => {
            let left_value = evaluate(&left, state)?;
            let right_value = evaluate(&right, state)?;
            match (left_value.clone(), right_value.clone()) {
                (Value::Integer(left), Value::Integer(right)) => {
                    if right == 0 {
                        Err(Error::InvalidValues(vec![right_value]))
                    } else {
                        Ok(Value::Float(left as f64 / right as f64))
                    }
                }
                _ => Err(Error::InvalidValues(vec![left_value, right_value])),
            }
        }
        Expression::If {
            condition,
            then_branch,
            else_branch,
        } => {
            let condition_value = evaluate(&condition, state)?;
            match condition_value {
                Value::Boolean(true) => evaluate(&then_branch, state),
                Value::Boolean(false) => evaluate(&else_branch, state),
                _ => Err(Error::InvalidValues(vec![condition_value])),
            }
        }
        Expression::Compare {
            left,
            operand,
            right,
        } => {
            let left_value = evaluate(&left, state)?;
            let right_value = evaluate(&right, state)?;
            match (left_value.clone(), right_value.clone()) {
                (Value::Integer(left), Value::Integer(right)) => match operand {
                    Operand::Equals => Ok(Value::Boolean(left == right)),
                    Operand::LessThan => Ok(Value::Boolean(left < right)),
                    Operand::GreaterThan => Ok(Value::Boolean(left > right)),
                },
                _ => Err(Error::InvalidValues(vec![left_value, right_value])),
            }
        }
        Expression::Primitive(primitive) => match primitive {
            Primitive::Integer(value) => Ok(Value::Integer(*value)),
            Primitive::String(value) => Ok(Value::String(value.clone())),
        },
        Expression::Let { variables, scope } => {
            let mut state = match state.clone() {
                Some(state) => state,
                None => HashMap::new(),
            };

            for (name, value) in variables {
                state.insert(name.to_string(), value.clone());
            }

            evaluate(&scope, &Some(state))
        }
        Expression::Use { variable } => match state {
            Some(state) => match state.get(variable) {
                Some(value) => evaluate(&value, &Some(state.clone())),
                None => Err(Error::UndefinedVariable(variable.to_string())),
            },
            None => Err(Error::UndefinedVariable(variable.to_string())),
        },
        Expression::For {
            variable,
            from,
            to,
            body,
        } => {
            let from_value = evaluate(&from, state)?;
            let to_value = evaluate(&to, state)?;
            let mut previous_value = state.clone();
            match (from_value.clone(), to_value.clone()) {
                (Value::Integer(from), Value::Integer(to)) => {
                    for i in from..=to {
                        let current_value = Rc::new(Expression::Primitive(Primitive::Integer(i)));
                        match state {
                            Some(_) => {
                                match previous_value.clone() {
                                    Some(prev) => {
                                        let mut new = prev.clone();
                                        new.insert(variable.to_string(), current_value);
                                        let state_value = evaluate(&body, &Some(prev.clone()))?;
                                        match state_value {
                                            Value::State(new_value) => previous_value = new_value,
                                            _ => (),
                                        }
                                    }
                                    None => {
                                        let mut state = HashMap::new();
                                        state.insert(variable.to_string(), current_value);
                                        evaluate(&body, &Some(state))?;
                                    }
                                };
                            }
                            None => {
                                let mut state = HashMap::new();
                                state.insert(variable.to_string(), current_value);
                                evaluate(&body, &Some(state))?;
                            }
                        }
                    }
                    Ok(Value::Unit)
                }
                _ => Err(Error::InvalidValues(vec![from_value, to_value])),
            }
        }
        Expression::Print { expression } => {
            let string = match evaluate(&expression, state)? {
                Value::String(val) => val,
                Value::Integer(val) => val.to_string(),
                Value::Float(val) => val.to_string(),
                Value::Boolean(val) => val.to_string(),
                Value::State(val) => {
                    let mut string = String::new();
                    for (key, value) in val.unwrap().iter() {
                        string.push_str(&format!(
                            "{} = {}\n",
                            key,
                            evaluate(&value, state).unwrap()
                        ));
                    }
                    string
                }
                Value::Unit => "".to_string(),
                Value::None => "None".to_string(),
            };
            println!("{}", string);
            Ok(Value::Unit)
        }
        Expression::Chain { left, right } => {
            let state_value = evaluate(&left, state)?;
            let resul = match state_value {
                Value::State(state) => evaluate(&right, &state),
                _ => evaluate(&right, state),
            };
            resul
        }
        Expression::Assign { variable, value } => match state.clone() {
            Some(mut state) => {
                let value = evaluate(&value, &Some(state.clone()))?;
                let prim = match value {
                    Value::Integer(value) => Primitive::Integer(value),
                    Value::String(value) => Primitive::String(value),
                    _ => panic!("unexpected value: {:?}", value),
                };
                state.insert(variable.clone(), Rc::new(Expression::Primitive(prim)));
                Ok(Value::State(Some(state)))
            }
            None => Err(Error::UndefinedVariable(variable.to_string())),
        },
    }
}
