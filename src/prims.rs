use std::ops::{Deref, DerefMut};

use crate::types::{BoxedData, Data};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Prims {
    Add,
    Sub,
    Mul,
    Div,
    Define,
    Lambda,
    Quote,
}

impl Prims {
    pub fn eval(&self, a: BoxedData, env: &mut BoxedData) -> BoxedData {
        match self {
            Prims::Add => Self::add(a, env),
            Prims::Sub => Self::sub(a, env),
            Prims::Mul => Self::mul(a, env),
            Prims::Div => Self::div(a, env),
            Prims::Define => Self::define(a, env),
            Prims::Lambda => Self::lambda(a, env),
            Prims::Quote => Self::quote(a, env),
        }
    }

    fn add(a: BoxedData, env: &mut BoxedData) -> BoxedData {
        let op1 = Data::car(a.clone());
        let op2 = Data::car(Data::cdr(a.clone()));
        if let (Data::Number(num1), Data::Number(num2)) =
            (Data::eval(op1, env).deref(), Data::eval(op2, env).deref())
        {
            Data::number(num1 + num2)
        } else {
            Data::err()
        }
    }

    fn sub(a: BoxedData, env: &mut BoxedData) -> BoxedData {
        let op1 = Data::car(a.clone());
        let op2 = Data::car(Data::cdr(a.clone()));
        if let (Data::Number(num1), Data::Number(num2)) =
            (Data::eval(op1, env).deref(), Data::eval(op2, env).deref())
        {
            Data::number(num1 - num2)
        } else {
            Data::err()
        }
    }

    fn mul(a: BoxedData, env: &mut BoxedData) -> BoxedData {
        todo!()
    }

    fn div(a: BoxedData, env: &mut BoxedData) -> BoxedData {
        todo!()
    }

    fn define(a: BoxedData, mut env: &mut BoxedData) -> BoxedData {
        let op1 = Data::car(a.clone());
        let op2 = Data::car(Data::cdr(a.clone()));
        let result = Data::pair(op1.clone(), Data::eval(op2.clone(), env), &mut env);
        let val = env.deref_mut();
        *val = result;
        return op1;
    }

    fn lambda(a: BoxedData, env: &mut BoxedData) -> BoxedData {
        let op1 = Data::car(a.clone());
        let op2 = Data::car(Data::cdr(a.clone()));
        return Data::closure(op1, op2, env);
    }

    fn quote(a: BoxedData, env: &mut BoxedData) -> BoxedData {
        todo!()
    }
}
