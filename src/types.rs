use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::LazyLock;

use crate::prims::Prims;

#[derive(PartialEq, Eq, Debug)]
pub enum Data {
    Cons(Arc<Data>, Arc<Data>),
    Closure(Arc<Data>, Arc<Data>),
    Prim(Prims),
    Number(usize),
    Atomic(String),
    Nil,
    Error,
}

pub type BoxedData = Arc<Data>;

impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Data::Cons(car, cdr) => {
                if Data::not(cdr.clone()) {
                    write!(f, "{}", car)
                } else {
                    write!(f, "( {} {} )", car, cdr)
                }
            }
            Data::Closure(car, cdr) => {
                write!(f, "( {} => {} )", car, cdr)
            }
            Data::Prim(prims) => {
                write!(f, "{:?}", prims)
            }
            Data::Number(num) => write!(f, "{}", num),
            Data::Atomic(atom) => write!(f, "{}", atom),
            Data::Nil => write!(f, "nil"),
            Data::Error => write!(f, "err"),
        }
    }
}

static PRIM_TABLE: LazyLock<HashMap<String, Prims>> = LazyLock::new(|| {
    let mut table = HashMap::new();
    table.insert("+".to_string(), Prims::Add);
    table.insert("-".to_string(), Prims::Sub);
    table.insert("*".to_string(), Prims::Mul);
    table.insert("/".to_string(), Prims::Div);
    table.insert("define".to_string(), Prims::Define);
    table.insert("lambda".to_string(), Prims::Lambda);
    table
});

pub static ENV: LazyLock<BoxedData> = LazyLock::new(|| Arc::new(Data::Nil));

impl Data {
    pub fn cons(car: BoxedData, cdr: BoxedData) -> BoxedData {
        Arc::new(Data::Cons(car, cdr))
    }
    pub fn number(num: usize) -> BoxedData {
        Arc::new(Data::Number(num))
    }
    pub fn atom(sym: &String) -> BoxedData {
        Arc::new(Data::Atomic(sym.clone()))
    }
    pub fn pair(a: BoxedData, b: BoxedData, env: &mut BoxedData) -> BoxedData {
        Self::cons(Self::cons(a, b), env.clone())
    }
    pub fn prim(sym: &String) -> Option<BoxedData> {
        let c = PRIM_TABLE.get(sym)?;
        Some(Arc::new(Data::Prim(c.clone())))
    }
    pub fn closure(param: BoxedData, body: BoxedData, env: &mut BoxedData) -> BoxedData {
        let g_env = ENV.clone();
        let mut pair_env = if Self::equ(g_env, env.clone()) {
            Self::nil()
        } else {
            env.clone()
        };
        let pair_result = Self::pair(param, body, &mut pair_env);
        if let Data::Cons(ref car, ref cdr) = *pair_result {
            Arc::new(Data::Closure(car.clone(), cdr.clone()))
        } else {
            Self::err()
        }
    }
    pub fn nil() -> BoxedData {
        Arc::new(Data::Nil)
    }
    pub fn err() -> BoxedData {
        Arc::new(Data::Error)
    }
    pub fn equ(a: BoxedData, b: BoxedData) -> bool {
        return *a == *b;
    }
    pub fn not(a: BoxedData) -> bool {
        return *a == Data::Nil;
    }
    pub fn car(a: BoxedData) -> BoxedData {
        if let Data::Cons(ref car, _) = *a {
            car.clone()
        } else if let Data::Closure(ref car, _) = *a {
            car.clone()
        } else {
            Self::err()
        }
    }
    pub fn cdr(a: BoxedData) -> BoxedData {
        if let Data::Cons(_, ref cdr) = *a {
            cdr.clone()
        } else if let Data::Closure(_, ref cdr) = *a {
            cdr.clone()
        } else {
            Self::err()
        }
    }
    pub fn assoc(var: BoxedData, env: &mut BoxedData) -> BoxedData {
        let mut env = env.clone();
        while let Data::Cons(car, _) = env.as_ref() {
            if *Data::car(car.clone()) == *var {
                return Data::cdr(car.clone());
            }
            env = Data::cdr(env);
        }
        return Self::err();
    }
    pub fn eval(var: BoxedData, env: &mut BoxedData) -> BoxedData {
        match var.deref() {
            Self::Atomic(_) => Self::assoc(var, env),
            Self::Closure(car, cdr) => Self::apply(Self::eval(car.clone(), env), cdr.clone(), env),
            Self::Cons(car, cdr) => Self::apply(Self::eval(car.clone(), env), cdr.clone(), env),
            _ => var,
        }
    }
    pub fn apply(clos: BoxedData, param: BoxedData, env: &mut BoxedData) -> BoxedData {
        match clos.deref() {
            Self::Prim(prim) => prim.eval(param, env),
            Self::Closure(_, _) => Self::reduce(clos, param, env),
            _ => Self::err(),
        }
    }
    pub fn evlist(var: BoxedData, env: &mut BoxedData) -> BoxedData {
        match var.deref() {
            Self::Cons(car, cdr) => {
                Self::cons(Self::eval(car.clone(), env), Self::evlist(cdr.clone(), env))
            }
            Self::Atomic(_) => Self::assoc(var, env),
            _ => Self::nil(),
        }
    }

    pub fn bind(param: BoxedData, values: BoxedData, env: &mut BoxedData) -> BoxedData {
        if Self::not(param.clone()) {
            env.clone()
        } else {
            if let Data::Cons(_, _) = param.deref() {
                Self::bind(
                    Self::cdr(param.clone()),
                    Self::cdr(values.clone()),
                    &mut Self::pair(Self::car(param), Self::car(values), env),
                )
            } else {
                Self::pair(param, values, env)
            }
        }
    }

    pub fn reduce(clos: BoxedData, param: BoxedData, env: &mut BoxedData) -> BoxedData {
        Self::eval(
            Self::cdr(Self::car(clos.clone())),
            &mut Self::bind(
                Self::car(Self::car(clos.clone())),
                Self::evlist(param, env),
                &mut {
                    if Self::not(Self::cdr(clos.clone())) {
                        return env.clone();
                    }
                    Self::cdr(clos)
                },
            ),
        )
    }
}
