#![allow(dead_code)]

use serde::Deserialize;
use std::fmt::{Display, Error};
use std::{fmt, fs};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Deserialize)]
pub struct Program {
    name: String,
    expression: Term,
    // todo!("location")
}

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind")]
pub enum Term {
    Int(Int),
    Str(Str),
    Binary(Box<Binary>),
    Print(Box<Print>),
    Bool(Bool),
    Tuple(Box<Tuple>),
    First(Box<First>),
    Second(Box<Second>),
    Var(Var),
    Call(Box<Call>),
    Function(Box<Function>),
    Let(Box<Let>),
    If(Box<If>),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Int {
    value: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Str {
    value: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Bool {
    value: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Print {
    value: Term,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tuple {
    first: Term,
    second: Term,
}

#[derive(Debug, Clone, Deserialize)]
pub struct First {
    value: Term,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Second {
    value: Term,
}

#[derive(Debug, Clone, Deserialize)]
pub struct If {
    condition: Term,
    then: Term,
    otherwise: Term,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Var {
    text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Function {
    parameters: Vec<Parameter>,
    value: Term,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Let {
    name: Parameter,
    value: Term,
    next: Term,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Parameter {
    text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Call {
    callee: Term,
    arguments: Vec<Term>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Binary {
    lhs: Term,
    op: BinaryOperator,
    rhs: Term,
}

#[derive(Debug, Default)]
pub struct Scope {
    parent: Option<Rc<Scope>>,
    current: Rc<RefCell<HashMap<String, Val>>>,
}

impl Scope {
    pub fn get(&self, var: &str) -> Option<Val> {
        self.current
            .borrow()
            .get(var)
            .cloned()
            .or_else(|| self.parent.as_ref()?.get(var))
    }

    pub fn set(&self, var: impl Into<String>, val: Val) {
        self.current.borrow_mut().insert(var.into(), val);
    }
}

impl Clone for Scope {
    fn clone(&self) -> Self {
        Scope {
            parent: Some(Rc::new(Scope {
                parent: self.parent.clone(),
                current: self.current.clone(),
            })),
            current: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum Val {
    Str(String),
    Int(i32),
    Bool(bool),
    Tuple((Box<Val>, Box<Val>)),
    Closure { func: Function, env: Scope },
}

impl Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Val::Int(i) => write!(f, "{i}"),
            Val::Bool(true) => write!(f, "true"),
            Val::Bool(false) => write!(f, "false"),
            Val::Str(s) => write!(f, "{s}"),
            Val::Tuple((fst, snd)) => write!(f, "({fst}, {snd})"),
            Val::Closure { .. } => write!(f, "<#closure>"),
        }
    }
}

fn main() {
    let file: String = fs::read_to_string("../rinha-de-compiler/files/fib.json").unwrap();
    let program: Program = serde_json::from_str(&file).unwrap();

    let term: Term = program.expression;
    let scope = Scope::default();

    let val = eval(term, &scope);
    println!("{:?}", val);
}

fn eval(term: Term, scope: &Scope) -> Result<Val, Error> {
    match term {
        Term::Int(number) => Ok(Val::Int(number.value)),
        Term::Str(string) => Ok(Val::Str(string.value)),
        Term::Bool(boolean) => Ok(Val::Bool(boolean.value)),
        Term::Print(print) => {
            let val = eval(print.value, scope)?;
            println!("{val}");
            Ok(val)
        }
        Term::Binary(binary) => {
            let lhs = eval(binary.lhs.clone(), scope)?;
            let rhs = eval(binary.rhs.clone(), scope)?;

            match binary.op {
                BinaryOperator::Add => match (lhs, rhs) {
                    (Val::Int(lhs), Val::Int(rhs)) => Ok(Val::Int(lhs + rhs)),
                    (lhs, rhs) => Ok(Val::Str(format!("{:?}{:?}", lhs, rhs))),
                },
                BinaryOperator::Sub => match (lhs, rhs) {
                    (Val::Int(lhs), Val::Int(rhs)) => Ok(Val::Int(lhs - rhs)),
                    _ => Err(Error),
                },

                BinaryOperator::Mul => match (lhs, rhs) {
                    (Val::Int(lhs), Val::Int(rhs)) => Ok(Val::Int(lhs * rhs)),
                    _ => Err(Error),
                },

                BinaryOperator::Div => match (lhs, rhs) {
                    (Val::Int(lhs), Val::Int(rhs)) => {
                        if rhs == 0 {
                            println!("error -> division by zero");
                            return Err(Error);
                        }

                        Ok(Val::Int(lhs / rhs))
                    }
                    _ => Err(Error),
                },

                BinaryOperator::Rem => match (lhs, rhs) {
                    (Val::Int(lhs), Val::Int(rhs)) => Ok(Val::Int(lhs % rhs)),
                    _ => Err(Error),
                },
                BinaryOperator::Eq => match (lhs, rhs) {
                    (Val::Int(lhs), Val::Int(rhs)) => Ok(Val::Bool(lhs == rhs)),
                    (Val::Str(lhs), Val::Str(rhs)) => Ok(Val::Bool(lhs == rhs)),
                    (Val::Bool(lhs), Val::Bool(rhs)) => Ok(Val::Bool(lhs == rhs)),
                    _ => Err(Error),
                },
                BinaryOperator::Neq => match (lhs, rhs) {
                    (Val::Int(lhs), Val::Int(rhs)) => Ok(Val::Bool(lhs != rhs)),
                    (Val::Str(lhs), Val::Str(rhs)) => Ok(Val::Bool(lhs != rhs)),
                    (Val::Bool(lhs), Val::Bool(rhs)) => Ok(Val::Bool(lhs != rhs)),
                    _ => Err(Error),
                },
                BinaryOperator::Lt => match (lhs, rhs) {
                    (Val::Int(lhs), Val::Int(rhs)) => Ok(Val::Bool(lhs < rhs)),
                    (Val::Str(lhs), Val::Str(rhs)) => Ok(Val::Bool(lhs < rhs)),
                    (Val::Bool(lhs), Val::Bool(rhs)) => Ok(Val::Bool(lhs < rhs)),
                    _ => Err(Error),
                },
                BinaryOperator::Gt => match (lhs, rhs) {
                    (Val::Int(lhs), Val::Int(rhs)) => Ok(Val::Bool(lhs > rhs)),
                    (Val::Str(lhs), Val::Str(rhs)) => Ok(Val::Bool(lhs > rhs)),
                    (Val::Bool(lhs), Val::Bool(rhs)) => Ok(Val::Bool(lhs > rhs)),
                    _ => Err(Error),
                },
                BinaryOperator::Lte => match (lhs, rhs) {
                    (Val::Int(lhs), Val::Int(rhs)) => Ok(Val::Bool(lhs <= rhs)),
                    (Val::Str(lhs), Val::Str(rhs)) => Ok(Val::Bool(lhs <= rhs)),
                    (Val::Bool(lhs), Val::Bool(rhs)) => Ok(Val::Bool(lhs <= rhs)),
                    _ => Err(Error),
                },
                BinaryOperator::Gte => match (lhs, rhs) {
                    (Val::Int(lhs), Val::Int(rhs)) => Ok(Val::Bool(lhs >= rhs)),
                    (Val::Str(lhs), Val::Str(rhs)) => Ok(Val::Bool(lhs >= rhs)),
                    (Val::Bool(lhs), Val::Bool(rhs)) => Ok(Val::Bool(lhs >= rhs)),
                    _ => Err(Error),
                },
                BinaryOperator::And => match (lhs, rhs) {
                    (Val::Bool(lhs), Val::Bool(rhs)) => Ok(Val::Bool(lhs && rhs)),
                    _ => Err(Error),
                },
                BinaryOperator::Or => match (lhs, rhs) {
                    (Val::Bool(lhs), Val::Bool(rhs)) => Ok(Val::Bool(lhs || rhs)),
                    _ => Err(Error),
                },
            }
        }
        Term::Tuple(tuple) => Ok(Val::Tuple((
            Box::from(eval(tuple.first, scope)?),
            Box::from(eval(tuple.second, scope)?),
        ))),
        Term::First(first) => match eval(first.value, scope)? {
            Val::Tuple((val, _)) => Ok(*val),
            _ => Err(Error),
        },
        Term::Second(second) => match eval(second.value, scope)? {
            Val::Tuple((_, val)) => Ok(*val),
            _ => Err(Error),
        },
        Term::If(if_term) => match eval(if_term.condition, scope)? {
            Val::Bool(true) => eval(if_term.then, scope),
            Val::Bool(false) => eval(if_term.otherwise, scope),
            _ => Err(Error),
        },
        Term::Var(var) => match scope.get(&var.text) {
            Some(val) => Ok(val),
            None => Err(Error),
        },
        Term::Let(lt) => {
            let name = lt.name.text;
            scope.set(name, eval(lt.value, scope)?);
            eval(lt.next, scope)
        }
        Term::Function(func) => Ok(Val::Closure {
            func: *func,
            env: scope.clone(),
        }),
        Term::Call(call) => match eval(call.callee, scope)? {
            Val::Closure { func, env } => {
                if call.arguments.len() != func.parameters.len() {
                    return Err(Error);
                }

                for (param, arg) in func.parameters.into_iter().zip(call.arguments) {
                    env.set(param.text, eval(arg, scope)?);
                }

                eval(func.value, &env)
            }
            _ => Err(Error),
        },
    }
}
