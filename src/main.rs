#![allow(dead_code)]
#![allow(unused)]

use serde::Deserialize;
use std::{boxed, fmt::Debug, fs};

#[derive(Debug, Deserialize)]
pub struct Program {
    name: String,
    expression: Term,
    // todo!("location")
}

#[derive(Debug, Deserialize)]
#[serde(tag = "kind")]
pub enum Term {
    Int(Int),
    Str(Str),
    Bool(Bool),
    Print(Print),
    Binary(Binary),
}

#[derive(Debug, Deserialize)]
pub struct Int {
    value: i32,
}

#[derive(Debug, Deserialize)]
pub struct Str {
    value: String,
}

#[derive(Debug, Deserialize)]
pub struct Bool {
    value: bool,
}

#[derive(Debug, Deserialize)]
pub struct Print {
    value: Box<Term>,
}

#[derive(Debug, Deserialize)]
pub struct Binary {
    lhs: Box<Term>,
    op: BinaryOperator,
    rhs: Box<Term>,
}

#[derive(Debug, Deserialize)]
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

    pub fn leq(self, lhs: Term, rhs: Term) -> bool {
        match (lhs, rhs) {
            (Term::Int(lhs), Term::Int(rhs)) => lhs.value <= rhs.value,
            (Term::Str(lhs), Term::Str(rhs)) => lhs.value <= rhs.value,
            (Term::Bool(lhs), Term::Bool(rhs)) => lhs.value <= rhs.value,
            _ => {
                panic!("leq - value not supported")
            }
        }
    }

    pub fn geq(self, lhs: Term, rhs: Term) -> bool {
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

#[derive(Debug)]
pub enum Val {
    Null,
    Str(String),
    Int(i32),
    Bool(bool),
    Binary(Binary),
}

fn main() {
    let file: String = fs::read_to_string("../rinha-de-compiler/files/calc.json").unwrap();
    let program: Program = serde_json::from_str(&file).unwrap();

    let term: Term = program.expression;

    eval(term);
}

fn eval(term: Term) -> Val {
    match term {
        Term::Int(number) => Val::Int(number.value),
        Term::Str(string) => Val::Str(string.value),
        Term::Bool(boolean) => Val::Bool(boolean.value),
        Term::Print(print) => {
            let val = eval(*print.value);
            match val {
                Val::Str(string) => println!("{}", string),
                Val::Int(number) => println!("{}", number),
                Val::Bool(boolean) => println!("{}", boolean),
                _ => {}
            }

            Val::Null
        }
        Term::Binary(binary) => match binary.op {
            BinaryOperator::Add => {
                let result = binary.op.add(*binary.lhs, *binary.rhs);
                match result {
                    AddResult::Int(number) => println!("{}", number),
                    AddResult::String(string) => println!("{}", string),
                };
                Val::Null
            }
            BinaryOperator::Sub => {
                let result = binary.op.sub(*binary.lhs, *binary.rhs);
                println!("{}", result);
                Val::Null
            }
            BinaryOperator::Mul => {
                let result = binary.op.mul(*binary.lhs, *binary.rhs);
                println!("{}", result);
                Val::Null
            }
            BinaryOperator::Div => {
                let result = binary.op.div(*binary.lhs, *binary.rhs);
                println!("{}", result);
                Val::Null
            }
            BinaryOperator::Rem => {
                let result = binary.op.rem(*binary.lhs, *binary.rhs);
                println!("{}", result);
                Val::Null
            }
            BinaryOperator::Eq => {
                let result = binary.op.eq(*binary.lhs, *binary.rhs);
                println!("{}", result);
                Val::Null
            }
            BinaryOperator::Neq => {
                let result = binary.op.neq(*binary.lhs, *binary.rhs);
                println!("{}", result);
                Val::Null
            }
            BinaryOperator::Lt => {
                let result = binary.op.lt(*binary.lhs, *binary.rhs);
                println!("{}", result);
                Val::Null
            }
            BinaryOperator::Gt => {
                let result = binary.op.gt(*binary.lhs, *binary.rhs);
                println!("{}", result);
                Val::Null
            }
            BinaryOperator::Lte => {
                let result = binary.op.leq(*binary.lhs, *binary.rhs);
                println!("{}", result);
                Val::Null
            }
            BinaryOperator::Gte => {
                let result = binary.op.geq(*binary.lhs, *binary.rhs);
                println!("{}", result);
                Val::Null
            }
            BinaryOperator::And => {
                let result = binary.op.and(*binary.lhs, *binary.rhs);
                println!("{}", result);
                Val::Null
            }
            BinaryOperator::Or => {
                let result = binary.op.or(*binary.lhs, *binary.rhs);
                println!("{}", result);
                Val::Null
            }

            _ => Val::Null,
        },
        _ => Val::Null,
    }
}
