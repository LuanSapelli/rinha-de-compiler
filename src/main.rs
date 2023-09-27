#![allow(dead_code)]

use serde::Deserialize;
use std::fmt::{Display, Error};
use std::{fmt, fs};

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

#[derive(Debug, Default, Clone, Deserialize)]
pub struct Scope {
    parent: Box<Option<Scope>>,
    current: Vec<(String, Val)>,
}

impl Scope {
    pub fn set(&mut self, name: String, val: Val) {
        self.current.push((name, val));
    }

    pub fn get(&self, name: String) -> Option<Val> {
        match self.current.iter().find(|(key, _)| key == &name) {
            Some((_, val)) => Some(val.clone()),
            None => match &*self.parent {
                Some(parent) => parent.get(name),
                None => None,
            },
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

#[derive(Debug)]
pub enum AddResult {
    Int(i32),
    String(String),
}

#[derive(Debug, Clone, Deserialize)]
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

    let val = eval(term, scope);
    println!("{:?}", val);
}

fn eval(term: Term, mut scope: Scope) -> Result<Val, Error> {
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
            let lhs = eval(binary.lhs.clone(), scope.clone())?;
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
            Box::from(eval(tuple.first, scope.clone())?),
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
        Term::If(if_term) => match eval(if_term.condition, scope.clone())? {
            Val::Bool(true) => eval(if_term.then, scope),
            Val::Bool(false) => eval(if_term.otherwise, scope),
            _ => Err(Error),
        },
        Term::Var(var) => match scope.get(var.text) {
            Some(val) => Ok(val),
            None => {
                //TODO -> fix
                println!("caindo aqui");
                Err(Error)
            }
        },
        Term::Let(lt) => {
            scope.set(lt.name.text, eval(lt.value, scope.clone())?);
            eval(lt.next, scope.clone())
        }
        Term::Function(func) => Ok(Val::Closure {
            func: *func,
            env: scope,
        }),
        Term::Call(call) => match eval(call.callee, scope.clone())? {
            Val::Closure { func, mut env } => {
                if call.arguments.len() != func.parameters.len() {
                    return Err(Error);
                }

                for (param, arg) in func.parameters.into_iter().zip(call.arguments) {
                    env.set(param.text, eval(arg, scope.clone())?);
                }
                eval(func.value, env)
            }
            _ => Err(Error),
        },
    }
}
