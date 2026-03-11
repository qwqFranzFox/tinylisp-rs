use std::ops::{Deref, DerefMut};

use crate::types::{BoxedData, Data, ENV};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Prims {
    Add,
    Sub,
    Mul,
    Div,
    Define,
    Lambda,
    Quote,
    If,
    Eq,
    Mod,
}

impl Prims {
    pub fn eval(&self, a: BoxedData, env: BoxedData) -> BoxedData {
        match self {
            Prims::Add => Self::add(a, env),
            Prims::Sub => Self::sub(a, env),
            Prims::Mul => Self::mul(a, env),
            Prims::Div => Self::div(a, env),
            Prims::Define => Self::define(a, env),
            Prims::Lambda => Self::lambda(a, env),
            Prims::Quote => Self::quote(a, env),
            Prims::If => Self::if_(a, env),
            Prims::Eq => Self::equ(a, env),
            Prims::Mod => Self::mod_(a, env),
        }
    }

    fn add(a: BoxedData, env: BoxedData) -> BoxedData {
        let op1 = Data::car(a.clone());
        let op2 = Data::car(Data::cdr(a.clone()));
        if let (Data::Number(num1), Data::Number(num2)) = (
            Data::eval(op1, env.clone()).deref(),
            Data::eval(op2, env).deref(),
        ) {
            Data::number(num1 + num2)
        } else {
            Data::err()
        }
    }

    fn sub(a: BoxedData, env: BoxedData) -> BoxedData {
        let op1 = Data::car(a.clone());
        let op2 = Data::car(Data::cdr(a.clone()));
        if let (Data::Number(num1), Data::Number(num2)) = (
            Data::eval(op1, env.clone()).deref(),
            Data::eval(op2, env).deref(),
        ) {
            Data::number(num1 - num2)
        } else {
            Data::err()
        }
    }

    fn mul(a: BoxedData, env: BoxedData) -> BoxedData {
        todo!()
    }

    fn div(a: BoxedData, env: BoxedData) -> BoxedData {
        todo!()
    }

    fn mod_(a: BoxedData, env: BoxedData) -> BoxedData {
        let op1 = Data::car(a.clone());
        let op2 = Data::car(Data::cdr(a.clone()));
        if let (Data::Number(num1), Data::Number(num2)) = (
            Data::eval(op1, env.clone()).deref(),
            Data::eval(op2, env).deref(),
        ) {
            Data::number(num1 % num2)
        } else {
            Data::err()
        }
    }

    fn define(a: BoxedData, env: BoxedData) -> BoxedData {
        let op1 = Data::car(a.clone());
        let op2 = Data::car(Data::cdr(a.clone()));
        let result = Data::eval(op2.clone(), env.clone());
        let mut global_env = ENV.lock().unwrap();
        let global_env = global_env.deref_mut();
        *global_env = Data::pair(op1.clone(), result, global_env.clone());
        return op1;
    }

    fn lambda(a: BoxedData, env: BoxedData) -> BoxedData {
        let op1 = Data::car(a.clone());
        let op2 = Data::car(Data::cdr(a.clone()));
        return Data::closure(op1, op2, env);
    }

    fn quote(a: BoxedData, _env: BoxedData) -> BoxedData {
        Data::car(a)
    }

    fn equ(a: BoxedData, env: BoxedData) -> BoxedData {
        let tru = Data::atom(&"#t".to_string());
        let op1 = Data::car(a.clone());
        let op2 = Data::car(Data::cdr(a.clone()));
        let op1 = Data::eval(op1, env.clone());
        let op2 = Data::eval(op2, env.clone());
        if Data::equ(op1, op2) {
            tru
        } else {
            Data::nil()
        }
    }
    fn if_(a: BoxedData, env: BoxedData) -> BoxedData {
        let cond = Data::car(a.clone());
        let op1 = Data::car(Data::cdr(a.clone()));
        let op2 = Data::car(Data::cdr(Data::cdr(a)));
        println!("{cond} {op1} {op2}");
        let cond = Data::eval(cond, env.clone());
        if !Data::not(cond) {
            Data::eval(op1, env)
        } else {
            Data::eval(op2, env)
        }
    }
}
