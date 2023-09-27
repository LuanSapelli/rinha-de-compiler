#![allow(dead_code)]
#![allow(unused)]

use serde::Deserialize;
use std::fmt::{Display, Error};
use std::ops::Deref;
use std::{boxed, fmt, fmt::Debug, fs};

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

fn main() {
    let file: String = fs::read_to_string("../rinha-de-compiler/files/calc.json").unwrap();
    let program: Program = serde_json::from_str(&file).unwrap();

    let term: Term = program.expression;

    let val = eval(term);
    println!("{:?}", val)
}

fn eval(term: Term) -> Result<Val, Error> {
    match term {
        Term::Int(number) => Ok(Val::Int(number.value)),
        Term::Str(string) => Ok(Val::Str(string.value)),
        Term::Bool(boolean) => Ok(Val::Bool(boolean.value)),
        Term::Print(print) => {
            let val = eval(*print.value)?;
            println!("{}", val);
            Ok(val)
        }
        Term::Binary(binary) => match binary.op {
            BinaryOperator::Add => match (*binary.lhs, *binary.rhs) {
                (Term::Int(lhs), Term::Int(rhs)) => Ok(Val::Int(lhs.value + rhs.value)),
                (lhs, rhs) => Ok(Val::Str(format!("{:?}{:?}", lhs, rhs))),
            },
            BinaryOperator::Sub => match (*binary.lhs, *binary.rhs) {
                (Term::Int(lhs), Term::Int(rhs)) => Ok(Val::Int(lhs.value - rhs.value)),
                _ => Err(Error),
            },

            BinaryOperator::Mul => match (*binary.lhs, *binary.rhs) {
                (Term::Int(lhs), Term::Int(rhs)) => Ok(Val::Int(lhs.value * rhs.value)),
                _ => Err(Error),
            },

            BinaryOperator::Div => match (*binary.lhs, *binary.rhs) {
                (Term::Int(lhs), Term::Int(rhs)) => Ok(Val::Int(lhs.value / rhs.value)),
                _ => Err(Error),
            },

            BinaryOperator::Rem => match (*binary.lhs, *binary.rhs) {
                (Term::Int(lhs), Term::Int(rhs)) => Ok(Val::Int(lhs.value % rhs.value)),
                _ => Err(Error),
            },
            BinaryOperator::Eq => match (*binary.lhs, *binary.rhs) {
                (Term::Int(lhs), Term::Int(rhs)) => Ok(Val::Bool(lhs.value == rhs.value)),
                (Term::Str(lhs), Term::Str(rhs)) => Ok(Val::Bool(lhs.value == rhs.value)),
                (Term::Bool(lhs), Term::Bool(rhs)) => Ok(Val::Bool(lhs.value == rhs.value)),
                _ => Err(Error),
            },
            BinaryOperator::Neq => match (*binary.lhs, *binary.rhs) {
                (Term::Int(lhs), Term::Int(rhs)) => Ok(Val::Bool(lhs.value != rhs.value)),
                (Term::Str(lhs), Term::Str(rhs)) => Ok(Val::Bool(lhs.value != rhs.value)),
                (Term::Bool(lhs), Term::Bool(rhs)) => Ok(Val::Bool(lhs.value != rhs.value)),
                _ => Err(Error),
            },
            BinaryOperator::Lt => match (*binary.lhs, *binary.rhs) {
                (Term::Int(lhs), Term::Int(rhs)) => Ok(Val::Bool(lhs.value < rhs.value)),
                (Term::Str(lhs), Term::Str(rhs)) => Ok(Val::Bool(lhs.value < rhs.value)),
                (Term::Bool(lhs), Term::Bool(rhs)) => Ok(Val::Bool(lhs.value < rhs.value)),
                _ => Err(Error),
            },
            BinaryOperator::Gt => match (*binary.lhs, *binary.rhs) {
                (Term::Int(lhs), Term::Int(rhs)) => Ok(Val::Bool(lhs.value > rhs.value)),
                (Term::Str(lhs), Term::Str(rhs)) => Ok(Val::Bool(lhs.value > rhs.value)),
                (Term::Bool(lhs), Term::Bool(rhs)) => Ok(Val::Bool(lhs.value > rhs.value)),
                _ => Err(Error),
            },
            BinaryOperator::Lte => match (*binary.lhs, *binary.rhs) {
                (Term::Int(lhs), Term::Int(rhs)) => Ok(Val::Bool(lhs.value <= rhs.value)),
                (Term::Str(lhs), Term::Str(rhs)) => Ok(Val::Bool(lhs.value <= rhs.value)),
                (Term::Bool(lhs), Term::Bool(rhs)) => Ok(Val::Bool(lhs.value <= rhs.value)),
                _ => Err(Error),
            },
            BinaryOperator::Gte => match (*binary.lhs, *binary.rhs) {
                (Term::Int(lhs), Term::Int(rhs)) => Ok(Val::Bool(lhs.value >= rhs.value)),
                (Term::Str(lhs), Term::Str(rhs)) => Ok(Val::Bool(lhs.value >= rhs.value)),
                (Term::Bool(lhs), Term::Bool(rhs)) => Ok(Val::Bool(lhs.value >= rhs.value)),
                _ => Err(Error),
            },
            BinaryOperator::And => match (*binary.lhs, *binary.rhs) {
                (Term::Bool(lhs), Term::Bool(rhs)) => Ok(Val::Bool(lhs.value && rhs.value)),
                _ => Err(Error),
            },
            BinaryOperator::Or => match (*binary.lhs, *binary.rhs) {
                (Term::Bool(lhs), Term::Bool(rhs)) => Ok(Val::Bool(lhs.value || rhs.value)),
                _ => Err(Error),
            },
            _ => Err(Error),
        },
        Term::Tuple(tuple) => Ok(Val::Tuple((
            Box::new(eval(tuple.first)?),
            Box::new(eval(tuple.second)?),
        ))),
        Term::First(first) => match eval(first.value)? {
            Val::Tuple((val, _)) => Ok(*val),
            _ => Err(Error),
        },
        Term::Second(second) => match eval(second.value)? {
            Val::Tuple((_, val)) => Ok(*val),
            _ => Err(Error),
        },
        _ => Ok(Val::Bool(true)),
    }
}
