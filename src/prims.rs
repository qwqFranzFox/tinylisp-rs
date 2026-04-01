use crate::data::{BoxedData, Data, ENV};
use crate::peri::PeriWrap;
use crate::ports::ToString;
use core::ops::Deref;

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
    Eval,
    Blink,
    Car,
    Cdr,
    List,
}

pub fn to_prim(s: &str) -> Option<Prims> {
    return match s {
        "+" => Some(Prims::Add),
        "-" => Some(Prims::Sub),
        "*" => Some(Prims::Mul),
        "/" => Some(Prims::Div),
        "mod" => Some(Prims::Mod),
        "if" => Some(Prims::If),
        "eq?" => Some(Prims::Eq),
        "define" => Some(Prims::Define),
        "lambda" => Some(Prims::Lambda),
        "quote" => Some(Prims::Quote),
        "eval" => Some(Prims::Eval),
        "blink" => Some(Prims::Blink),
        "car" => Some(Prims::Car),
        "cdr" => Some(Prims::Cdr),
        "list" => Some(Prims::List),
        _ => None,
    };
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
            Prims::Mod => Self::modular(a, env),
            Prims::Eval => Self::ev(a, env),
            Prims::Blink => Self::blink(a, env),
            Prims::Car => Self::car(a, env),
            Prims::Cdr => Self::cdr(a, env),
            Prims::List => Self::list(a, env),
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
        let op1 = Data::car(a.clone());
        let op2 = Data::car(Data::cdr(a.clone()));
        if let (Data::Number(num1), Data::Number(num2)) = (
            Data::eval(op1, env.clone()).deref(),
            Data::eval(op2, env).deref(),
        ) {
            Data::number(num1 * num2)
        } else {
            Data::err()
        }
    }

    fn div(a: BoxedData, env: BoxedData) -> BoxedData {
        let op1 = Data::car(a.clone());
        let op2 = Data::car(Data::cdr(a.clone()));
        if let (Data::Number(num1), Data::Number(num2)) = (
            Data::eval(op1, env.clone()).deref(),
            Data::eval(op2, env).deref(),
        ) {
            Data::number(num1 / num2)
        } else {
            Data::err()
        }
    }

    fn modular(a: BoxedData, env: BoxedData) -> BoxedData {
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
        let global_env = ENV.read().clone();
        let result = Data::pair(op1.clone(), result, global_env.clone());
        {
            let mut global_env = ENV.write();
            *global_env = result;
        }
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
        let cond = Data::eval(cond, env.clone());
        if !Data::not(cond) {
            Data::eval(op1, env)
        } else {
            Data::eval(op2, env)
        }
    }

    fn ev(a: BoxedData, env: BoxedData) -> BoxedData {
        return Data::eval(a, env);
    }

    fn blink(_a: BoxedData, _env: BoxedData) -> BoxedData {
        // TODO: need to reconsider the implemetation of this primitive.
        // As the problems occured in previous development, delays are usually bad
        // for the repl loop and the usb poll loop.
        // Current imagine: use the hardware timer and a queue. The queue contains
        // blint events, and each time the timer interrupt arrives, if the queue
        // was not empty, the handler will set a new timer based on the
        // subsequent event.
        return Data::err();
    }

    fn car(a: BoxedData, env: BoxedData) -> BoxedData {
        return Data::car(Data::eval(a, env));
    }

    fn cdr(a: BoxedData, env: BoxedData) -> BoxedData {
        return Data::cdr(Data::eval(a, env));
    }

    fn list(a: BoxedData, env: BoxedData) -> BoxedData {
        if Data::equ(a.clone(), Data::nil()) {
            return a;
        } else {
            let op1 = Data::car(a.clone());
            return Data::cons(Data::eval(op1, env.clone()), Self::list(Data::cdr(a), env));
        }
    }
}
