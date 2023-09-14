use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct File {
    name: String,
    expression: Term,
    // todo!("location")
}

pub struct Print {}

#[derive(Debug, Deserialize)]
pub enum Term {
    Print(Print),
}

#[derive(Debug)]
pub enum Val {
    Null,
    Str(String),
    Int(i32),
    Bool(bool),
}

fn main() {}

fn eval(program: Term) -> Val {
    todo!("eval")
}
