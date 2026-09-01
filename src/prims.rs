use crate::data::{Data, DataImpl, LispContext};
// use crate::ports::ToString;
// use core::ops::Deref;

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
    pub fn eval(&self, context: &mut LispContext, a: Data, env: Data) -> Data {
        match self {
            Prims::Add => Self::add(context, a, env),
            Prims::Sub => Self::sub(context, a, env),
            Prims::Mul => Self::mul(context, a, env),
            Prims::Div => Self::div(context, a, env),
            Prims::Define => Self::define(context, a, env),
            Prims::Lambda => Self::lambda(context, a, env),
            Prims::Quote => Self::quote(context, a, env),
            Prims::If => Self::if_(context, a, env),
            Prims::Eq => Self::equ(context, a, env),
            Prims::Mod => Self::modular(context, a, env),
            Prims::Eval => Self::ev(context, a, env),
            Prims::Blink => Self::blink(context, a, env),
            Prims::Car => Self::car(context, a, env),
            Prims::Cdr => Self::cdr(context, a, env),
            Prims::List => Self::list(context, a, env),
        }
    }

    fn add(context: &mut LispContext, a: Data, env: Data) -> Data {
        let op1 = context.car(a.clone());
        let op2 = context.car(context.cdr(a.clone()));
        let val1 = context.eval(op1, env);
        let val2 = context.eval(op2, env);
        if let (DataImpl::Number(num1), DataImpl::Number(num2)) = (
            context.get_impl(val1).unwrap(),
            context.get_impl(val2).unwrap(),
        ) {
            context.number(num1 + num2)
        } else {
            context.err()
        }
    }

    fn sub(context: &mut LispContext, a: Data, env: Data) -> Data {
        let op1 = context.car(a.clone());
        let op2 = context.car(context.cdr(a.clone()));
        let val1 = context.eval(op1, env);
        let val2 = context.eval(op2, env);
        if let (DataImpl::Number(num1), DataImpl::Number(num2)) = (
            context.get_impl(val1).unwrap(),
            context.get_impl(val2).unwrap(),
        ) {
            context.number(num1 - num2)
        } else {
            context.err()
        }
    }

    fn mul(context: &mut LispContext, a: Data, env: Data) -> Data {
        let op1 = context.car(a.clone());
        let op2 = context.car(context.cdr(a.clone()));
        let val1 = context.eval(op1, env);
        let val2 = context.eval(op2, env);
        if let (DataImpl::Number(num1), DataImpl::Number(num2)) = (
            context.get_impl(val1).unwrap(),
            context.get_impl(val2).unwrap(),
        ) {
            context.number(num1 * num2)
        } else {
            context.err()
        }
    }

    fn div(context: &mut LispContext, a: Data, env: Data) -> Data {
        let op1 = context.car(a.clone());
        let op2 = context.car(context.cdr(a.clone()));
        let val1 = context.eval(op1, env);
        let val2 = context.eval(op2, env);
        if let (DataImpl::Number(num1), DataImpl::Number(num2)) = (
            context.get_impl(val1).unwrap(),
            context.get_impl(val2).unwrap(),
        ) {
            context.number(num1 / num2)
        } else {
            context.err()
        }
    }

    fn modular(context: &mut LispContext, a: Data, env: Data) -> Data {
        let op1 = context.car(a.clone());
        let op2 = context.car(context.cdr(a.clone()));
        let val1 = context.eval(op1, env);
        let val2 = context.eval(op2, env);
        if let (DataImpl::Number(num1), DataImpl::Number(num2)) = (
            context.get_impl(val1).unwrap(),
            context.get_impl(val2).unwrap(),
        ) {
            context.number(num1 % num2)
        } else {
            context.err()
        }
    }

    fn define(context: &mut LispContext, a: Data, env: Data) -> Data {
        let op1 = context.car(a.clone());
        let op2 = context.car(context.cdr(a.clone()));
        let result = context.eval(op2.clone(), env.clone());
        let global_env = context.get_env();
        let result = context.pair(op1.clone(), result, global_env.clone());
        context.set_env(result);
        return op1;
    }

    fn lambda(context: &mut LispContext, a: Data, env: Data) -> Data {
        let op1 = context.car(a.clone());
        let op2 = context.car(context.cdr(a.clone()));
        return context.closure(op1, op2, env);
    }

    fn quote(context: &mut LispContext, a: Data, _env: Data) -> Data {
        context.car(a)
    }

    fn equ(context: &mut LispContext, a: Data, env: Data) -> Data {
        let tru = context.get_tru();
        let op1 = context.car(a.clone());
        let op2 = context.car(context.cdr(a.clone()));
        let op1 = context.eval(op1, env.clone());
        let op2 = context.eval(op2, env.clone());
        if context.get_impl(op1).unwrap() == context.get_impl(op2).unwrap() {
            tru
        } else {
            context.nil()
        }
    }
    fn if_(context: &mut LispContext, a: Data, env: Data) -> Data {
        let cond = context.car(a.clone());
        let op1 = context.car(context.cdr(a.clone()));
        let op2 = context.car(context.cdr(context.cdr(a)));
        let cond = context.eval(cond, env.clone());
        if !context.not(cond) {
            context.eval(op1, env)
        } else {
            context.eval(op2, env)
        }
    }

    fn ev(context: &mut LispContext, a: Data, env: Data) -> Data {
        return context.eval(a, env);
    }

    fn blink(context: &mut LispContext, _a: Data, _env: Data) -> Data {
        // TODO: need to reconsider the implemetation of this primitive.
        // As the problems occured in previous development, delays are usually bad
        // for the repl loop and the usb poll loop.
        // Current imagine: use the hardware timer and a queue. The queue contains
        // blink events, and each time the timer interrupt arrives, if the queue
        // was not empty, the handler will set a new timer based on the
        // subsequent event.
        return context.err();
    }

    fn car(context: &mut LispContext, a: Data, env: Data) -> Data {
        let eval_res = context.eval(a, env);
        return context.car(eval_res);
    }

    fn cdr(context: &mut LispContext, a: Data, env: Data) -> Data {
        let eval_res = context.eval(a, env);
        return context.cdr(eval_res);
    }

    fn list(context: &mut LispContext, a: Data, env: Data) -> Data {
        if a == context.nil() {
            return a;
        } else {
            let op1 = context.car(a.clone());
            let eval_result = context.eval(op1, env.clone());
            let cdr = Self::list(context, context.cdr(a), env);
            return context.cons(eval_result, cdr);
        }
    }
}
