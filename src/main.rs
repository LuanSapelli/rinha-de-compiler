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
    Print(Print),
    Bool(Bool),
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
pub struct Print {
    value: Box<Term>,
}

#[derive(Debug, Deserialize)]
pub struct Bool {
    value: bool,
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
    Mod,
    Eq,
    Neq,
    Lt,
    Gt,
    Leq,
    Geq,
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
        match lhs {
            Term::Int(lhs) => match rhs {
                Term::Int(rhs) => AddResult::Int(lhs.value + rhs.value),
                Term::Str(rhs) => AddResult::String(ToString::to_string(&lhs.value) + &rhs.value),
                _ => {
                    panic!("add - value not supported")
                }
            },
            Term::Str(lhs) => match rhs {
                Term::Int(rhs) => AddResult::String(lhs.value + &ToString::to_string(&rhs.value)),
                Term::Str(rhs) => AddResult::String(lhs.value + &rhs.value),
                _ => panic!("add - value not supported"),
            },
            _ => {
                panic!("add - value not supported")
            }
        }
    }


    // pub fn sub (self, lhs: Term, rhs: Term) -> i32 {
    //     match lhs {
    //         Term::Int(lhs) => match rhs {
    //             Term::Int(rhs) => lhs.value - rhs.value,
    //             _ => {
    //                 panic!("sub - value not supported")
    //             }
    //         },
    //         _ => {
    //             panic!("sub - value not supported")
    //         }
    //     }
    // }
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
                _ => {

                }
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
            // BinaryOperator::Sub => {
            //     let result = binary.op.sub(*binary.lhs, *binary.rhs);
            //     println!("{}", result);
            //
            //     Val::Null
            // },

            // todo!("Missing BinaryOperators")
            _ => {
                Val::Null
            }
        },
        _ => {
            Val::Null
        }
    }
}
