use crate::prims::Prims;
use crate::prims::to_prim;
use slotmap::{self, SlotMap};
use std::fmt::Display;
use std::fmt::Formatter;

pub type IntType = isize;

slotmap::new_key_type! {
   pub struct Data;
}

pub struct LispContext {
    env: Data,
    nil: Data,
    err: Data,
    tru: Data,
    alloc: slotmap::SlotMap<Data, DataImpl>,
}

impl LispContext {
    pub fn new() -> LispContext {
        let mut alloc = SlotMap::with_key();
        let err = alloc.insert(DataImpl::Atomic("err".to_string()));
        let nil = alloc.insert(DataImpl::Atomic("nil".to_string()));
        let tru = alloc.insert(DataImpl::Atomic("tru".to_string()));
        let env = alloc.insert(DataImpl::Cons(nil, nil));
        LispContext {
            env,
            alloc,
            nil,
            err,
            tru,
        }
    }
    pub fn get_env(&self) -> Data {
        self.env
    }
    pub fn set_env(&mut self, env: Data) {
        self.env = env;
    }
    pub fn get_tru(&self) -> Data {
        self.tru
    }
    pub fn get_err(&self) -> Data {
        self.err
    }

    pub fn cons(&mut self, car: Data, cdr: Data) -> Data {
        self.alloc.insert(DataImpl::Cons(car, cdr))
    }
    pub fn number(&mut self, num: IntType) -> Data {
        self.alloc.insert(DataImpl::Number(num))
    }
    pub fn atom(&mut self, sym: &String) -> Data {
        self.alloc.insert(DataImpl::Atomic(sym.clone()))
    }
    pub fn pair(&mut self, a: Data, b: Data, env: Data) -> Data {
        let _a = self.cons(a, b);
        self.cons(_a, env)
    }
    pub fn prim(&mut self, sym: &String) -> Option<Data> {
        let c = to_prim(sym)?;
        Some(self.alloc.insert(DataImpl::Prim(c.clone())))
    }
    pub fn closure(&mut self, param: Data, body: Data, env: Data) -> Data {
        let g_env = self.env;
        let pair_env = if g_env == env {
            self.nil()
        } else {
            env.clone()
        };
        self.pair(param.clone(), body.clone(), pair_env.clone());
        let new = self.cons(param, body);
        self.alloc.insert(DataImpl::Closure(new, pair_env))
    }
    pub fn nil(&self) -> Data {
        self.nil
    }
    pub fn err(&self) -> Data {
        self.err
    }
    pub fn not(&self, a: Data) -> bool {
        a == self.nil
    }
    pub fn get_impl(&self, a: Data) -> Option<DataImpl> {
        // NOTE:currently use cloned() to keep the code clean.
        self.alloc.get(a).cloned()
    }
    pub fn car(&self, a: Data) -> Data {
        let a_impl = self.get_impl(a).unwrap();
        if let DataImpl::Cons(car, _) = a_impl {
            car.clone()
        } else if let DataImpl::Closure(car, _) = a_impl {
            car.clone()
        } else {
            self.err
        }
    }
    pub fn cdr(&self, a: Data) -> Data {
        let a_impl = self.get_impl(a).unwrap();
        if let DataImpl::Cons(_, cdr) = a_impl {
            cdr.clone()
        } else if let DataImpl::Closure(_, cdr) = a_impl {
            cdr.clone()
        } else {
            self.err
        }
    }
    fn assoc(&mut self, var: Data, env: Data) -> Data {
        let mut env = env.clone();
        while let DataImpl::Cons(car, _) = self.get_impl(env).unwrap() {
            if self.get_impl(self.car(car.clone())) == self.get_impl(var) {
                var.dump(self, "found match");
                self.car(car.clone()).dump(self, "matching content");
                return self.cdr(car);
            }
            env = self.cdr(env);
        }
        return self.err;
    }
    pub fn eval(&mut self, var: Data, env: Data) -> Data {
        var.dump(self, "evaluating");
        env.dump(self, "env");
        match self.get_impl(var).unwrap() {
            DataImpl::Atomic(_) => self.assoc(var, env.clone()),
            DataImpl::Cons(car, cdr) => {
                let eval_result = self.eval(car.clone(), env.clone());
                self.apply(eval_result, cdr.clone(), env)
            }
            _ => var,
        }
    }

    fn apply(&mut self, clos: Data, param: Data, env: Data) -> Data {
        match self.get_impl(clos).unwrap() {
            DataImpl::Prim(ref prim) => prim.eval(self, param, env),
            DataImpl::Closure(_, _) => self.reduce(clos, param, env),
            _ => self.err(),
        }
    }
    pub fn evlist(&mut self, var: Data, env: Data) -> Data {
        let value = self.get_impl(var).unwrap();
        match value {
            DataImpl::Cons(car, cdr) => {
                let eval_res = self.eval(car.clone(), env.clone());
                let evlist_res = self.evlist(cdr.clone(), env);
                self.cons(eval_res, evlist_res)
            }
            DataImpl::Atomic(_) => self.assoc(var, env),
            _ => self.nil(),
        }
    }
    fn bind(&mut self, param: Data, values: Data, env: Data) -> Data {
        if self.not(param.clone()) {
            env.clone()
        } else {
            if let DataImpl::Cons(_, _) = self.get_impl(param).unwrap() {
                let pair = self.pair(self.car(param), self.car(values), env);
                self.bind(self.cdr(param.clone()), self.cdr(values.clone()), pair)
            } else {
                self.pair(param, values, env)
            }
        }
    }
    fn reduce(&mut self, clos: Data, param: Data, env: Data) -> Data {
        clos.dump(self, "reduce: clos");
        param.dump(self, "reduce: param");
        env.dump(self, "reduce: env");
        let body = self.cdr(self.car(clos.clone()));
        let params = self.car(self.car(clos.clone()));
        let values = self.evlist(param, env);
        let env = self.bind(params, values, {
            if self.not(self.cdr(clos.clone())) {
                let g_env = self.env;
                g_env
            } else {
                self.cdr(clos)
            }
        });
        self.eval(body, env).dump(self, "reduce result")
    }
}

impl Data {
    pub fn debug(self, context: &'_ LispContext) -> DataDebug<'_> {
        DataDebug {
            key: self,
            map: &context.alloc,
        }
    }
    pub fn dump(self, _context: &'_ LispContext, _message: &'_ str) -> Data {
        // println!("{}: {}", _message, self.clone().debug(_context));
        self
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum DataImpl {
    Cons(Data, Data),
    Closure(Data, Data),
    Prim(Prims),
    Number(IntType),
    Atomic(String),
}

impl Display for DataImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataImpl::Cons(car, cdr) => {
                write!(f, "( {:?} {:?} )", car, cdr)
            }
            DataImpl::Closure(car, cdr) => {
                write!(f, "( {:?} => {:?} )", car, cdr)
            }
            DataImpl::Prim(prims) => {
                write!(f, "{:?}", prims)
            }
            DataImpl::Number(num) => write!(f, "{}", num),
            DataImpl::Atomic(atom) => write!(f, "{}", atom),
        }
    }
}

pub struct Lisp {
    context: LispContext,
}

impl Lisp {
    pub fn new() -> Lisp {
        Lisp {
            context: LispContext::new(),
        }
    }
    pub fn get_context_mut(&mut self) -> &mut LispContext {
        &mut self.context
    }
    pub fn eval(&mut self, code: Data) -> Data {
        self.context.eval(code, self.context.get_env())
    }
}

pub struct DataDebug<'a> {
    key: Data,
    map: &'a SlotMap<Data, DataImpl>,
}

impl Display for DataDebug<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.fmt_inner(self.key, f)
    }
}

impl DataDebug<'_> {
    fn fmt_inner(&self, key: Data, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self.map.get(key) {
            Some(val) => val,
            None => return write!(f, "<invalid key {:?}>", key),
        };

        match val {
            DataImpl::Number(num) => {
                write!(f, "Number<{}>", num)
            }
            DataImpl::Cons(car, cdr) => {
                write!(f, "Cons<")?;
                self.fmt_inner(*car, f)?;
                write!(f, ", ")?;
                self.fmt_inner(*cdr, f)?;
                write!(f, ">")
            }
            DataImpl::Closure(car, cdr) => {
                write!(f, "Closure<")?;
                self.fmt_inner(*car, f)?;
                write!(f, ", ")?;
                self.fmt_inner(*cdr, f)?;
                write!(f, ">")
            }
            DataImpl::Prim(prim) => {
                write!(f, "Prim<{:?}>", prim)
            }
            DataImpl::Atomic(atom) => {
                write!(f, "Atomic<{}>", atom)
            }
        }
    }
}
