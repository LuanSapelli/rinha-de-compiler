#![allow(dead_code)]
#![allow(unused)]

use serde::Deserialize;
use std::fmt::{Display, Error};
use std::ops::Deref;
use std::{boxed, fmt, fmt::Debug, fs};
use std::ffi::NulError;

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
    Binary(Binary),
    Print(Print),
    Bool(Bool),
    Tuple(Box<Tuple>),
    First(Box<First>),
    Second(Box<Second>),
    Var(Var),
    // Call(Call),
    // Function(Function),
    // Let(Let),
    // If(If),
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
    value: Box<Term>,
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
pub struct Var {
    text: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Binary {
    lhs: Box<Term>,
    op: BinaryOperator,
    rhs: Box<Term>,
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

impl BinaryOperator {
    pub fn add(self, lhs: Term, rhs: Term) -> AddResult {
        match (lhs, rhs) {
            (Term::Int(lhs), Term::Int(rhs)) => AddResult::Int(lhs.value + rhs.value),
            (Term::Str(lhs), Term::Str(rhs)) => AddResult::String(lhs.value + &rhs.value),
            (Term::Int(lhs), Term::Str(rhs)) => {
                AddResult::String(ToString::to_string(&lhs.value) + &rhs.value)
            }
            (Term::Str(lhs), Term::Int(rhs)) => {
                AddResult::String(lhs.value + &ToString::to_string(&rhs.value))
            }

            _ => {
                panic!("add - value not supported")
            }
        }
    }

    pub fn sub(self, lhs: Term, rhs: Term) -> i32 {
        match (lhs, rhs) {
            (Term::Int(lhs), Term::Int(rhs)) => lhs.value - rhs.value,
            _ => {
                panic!("sub - value not supported")
            }
        }
    }

    pub fn mul(self, lhs: Term, rhs: Term) -> i32 {
        match (lhs, rhs) {
            (Term::Int(lhs), Term::Int(rhs)) => lhs.value * rhs.value,
            _ => {
                panic!("mul - value not supported")
            }
        }
    }

    pub fn div(self, lhs: Term, rhs: Term) -> i32 {
        match (lhs, rhs) {
            (Term::Int(lhs), Term::Int(rhs)) => {
                if rhs.value == 0 {
                    panic!("div - division by zero")
                }

                lhs.value / rhs.value
            }
            _ => {
                panic!("div - value not supported")
            }
        }
    }

    pub fn rem(self, lhs: Term, rhs: Term) -> i32 {
        match (lhs, rhs) {
            (Term::Int(lhs), Term::Int(rhs)) => lhs.value % rhs.value,
            _ => {
                panic!("mod - value not supported")
            }
        }
    }

    pub fn eq(self, lhs: Term, rhs: Term) -> bool {
        // todo! 2 == 1 + 1 scenario
        match (lhs, rhs) {
            (Term::Int(lhs), Term::Int(rhs)) => lhs.value == rhs.value,
            (Term::Str(lhs), Term::Str(rhs)) => lhs.value == rhs.value,
            (Term::Bool(lhs), Term::Bool(rhs)) => lhs.value == rhs.value,
            _ => {
                panic!("eq - value not supported")
            }
        }
    }

    pub fn neq(self, lhs: Term, rhs: Term) -> bool {
        // todo! 2 == 1 + 1 scenario
        match (lhs, rhs) {
            (Term::Int(lhs), Term::Int(rhs)) => lhs.value != rhs.value,
            (Term::Str(lhs), Term::Str(rhs)) => lhs.value != rhs.value,
            (Term::Bool(lhs), Term::Bool(rhs)) => lhs.value != rhs.value,
            _ => {
                panic!("neq - value not supported")
            }
        }
    }

    pub fn lt(self, lhs: Term, rhs: Term) -> bool {
        match (lhs, rhs) {
            (Term::Int(lhs), Term::Int(rhs)) => lhs.value < rhs.value,
            (Term::Str(lhs), Term::Str(rhs)) => lhs.value < rhs.value,
            (Term::Bool(lhs), Term::Bool(rhs)) => lhs.value < rhs.value,
            _ => {
                panic!("lt - value not supported")
            }
        }
    }

    pub fn gt(self, lhs: Term, rhs: Term) -> bool {
        match (lhs, rhs) {
            (Term::Int(lhs), Term::Int(rhs)) => lhs.value > rhs.value,
            (Term::Str(lhs), Term::Str(rhs)) => lhs.value > rhs.value,
            (Term::Bool(lhs), Term::Bool(rhs)) => lhs.value > rhs.value,
            _ => {
                panic!("gt - value not supported")
            }
        }
    }

    pub fn lte(self, lhs: Term, rhs: Term) -> bool {
        match (lhs, rhs) {
            (Term::Int(lhs), Term::Int(rhs)) => lhs.value <= rhs.value,
            (Term::Str(lhs), Term::Str(rhs)) => lhs.value <= rhs.value,
            (Term::Bool(lhs), Term::Bool(rhs)) => lhs.value <= rhs.value,
            _ => {
                panic!("leq - value not supported")
            }
        }
    }

    pub fn gte(self, lhs: Term, rhs: Term) -> bool {
        match (lhs, rhs) {
            (Term::Int(lhs), Term::Int(rhs)) => lhs.value >= rhs.value,
            (Term::Str(lhs), Term::Str(rhs)) => lhs.value >= rhs.value,
            (Term::Bool(lhs), Term::Bool(rhs)) => lhs.value >= rhs.value,
            _ => {
                panic!("geq - value not supported")
            }
        }
    }

    pub fn and(self, lhs: Term, rhs: Term) -> bool {
        match (lhs, rhs) {
            (Term::Bool(lhs), Term::Bool(rhs)) => lhs.value && rhs.value,
            _ => {
                panic!("and - value not supported")
            }
        }
    }

    pub fn or(self, lhs: Term, rhs: Term) -> bool {
        match (lhs, rhs) {
            (Term::Bool(lhs), Term::Bool(rhs)) => lhs.value || rhs.value,
            _ => {
                panic!("or - value not supported")
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum Val {
    Str(String),
    Int(i32),
    Bool(bool),
    Tuple((Box<Val>, Box<Val>)),
}

impl Display for Val {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Val::Int(i) => write!(f, "{i}"),
            Val::Bool(true) => write!(f, "true"),
            Val::Bool(false) => write!(f, "false"),
            Val::Str(s) => write!(f, "{s}"),
            Val::Tuple((fst, snd)) => write!(f, "({fst}, {snd})"),
        }
    }
}

#[derive(Clone)]
pub struct Scope {
    parent: Option<Box<Scope>>,
    values: Vec<(String, Term)>,
}

fn main() {
    let file: String = fs::read_to_string("../rinha-de-compiler/files/calc.json").unwrap();
    let program: Program = serde_json::from_str(&file).unwrap();

    let term: Term = program.expression;

    let scope = Scope {
        parent: None,
        values: vec![],
    };

    let val = eval(term, scope);
    println!("{:?}", val)
}

fn eval(term: Term, mut scope: Scope) -> Result<Val, Error> {
    match term {
        Term::Int(number) => Ok(Val::Int(number.value)),
        Term::Str(string) => Ok(Val::Str(string.value)),
        Term::Bool(boolean) => Ok(Val::Bool(boolean.value)),
        Term::Print(print) => {
            let val = eval(*print.value, scope)?;
            println!("{}", val);
            Ok(val)
        }
        Term::Binary(binary) => match binary.op {
            BinaryOperator::Add => match binary.op.add(*binary.lhs, *binary.rhs) {
                AddResult::Int(result) => Ok(Val::Int(result)),
                AddResult::String(result) => Ok(Val::Str(result)),
            },
            BinaryOperator::Sub => {
                Ok(Val::Int(binary.op.sub(*binary.lhs, *binary.rhs)))
            }
            BinaryOperator::Mul => {
                let result = binary.op.mul(*binary.lhs, *binary.rhs);
                Ok(Val::Int(result))
            }
            BinaryOperator::Div => {
                let result = binary.op.div(*binary.lhs, *binary.rhs);
                Ok(Val::Int(result))
            }
            BinaryOperator::Rem => {
                let result = binary.op.rem(*binary.lhs, *binary.rhs);
                Ok(Val::Int(result))
            }
            BinaryOperator::Eq => {
                let result = binary.op.eq(*binary.lhs, *binary.rhs);
                Ok(Val::Bool(result))
            }
            BinaryOperator::Neq => {
                let result = binary.op.neq(*binary.lhs, *binary.rhs);
                Ok(Val::Bool(result))
            }
            BinaryOperator::Lt => {
                let result = binary.op.lt(*binary.lhs, *binary.rhs);
                Ok(Val::Bool(result))
            }
            BinaryOperator::Gt => {
                let result = binary.op.gt(*binary.lhs, *binary.rhs);
                Ok(Val::Bool(result))
            }
            BinaryOperator::Lte => {
                let result = binary.op.lte(*binary.lhs, *binary.rhs);
                Ok(Val::Bool(result))
            }
            BinaryOperator::Gte => {
                let result = binary.op.gte(*binary.lhs, *binary.rhs);
                Ok(Val::Bool(result))
            }
            BinaryOperator::And => {
                let result = binary.op.and(*binary.lhs, *binary.rhs);
                Ok(Val::Bool(result))
            }
            BinaryOperator::Or => {
                let result = binary.op.or(*binary.lhs, *binary.rhs);
                Ok(Val::Bool(result))
            }
        },
        Term::Tuple(tuple) => Ok(Val::Tuple((
            Box::new(eval(tuple.first, scope.clone())?),
            Box::new(eval(tuple.second, scope.clone())?),
        ))),
        Term::First(first) => match eval(first.value, scope)? {
            Val::Tuple((val, _)) => Ok(*val),
            _ => Err(Error),
        },
        Term::Second(second) => match eval(second.value, scope)? {
            Val::Tuple((_, val)) => Ok(*val),
            _ => Err(Error),
        },
        _ => Ok(Val::Bool(true)),
    }
}
