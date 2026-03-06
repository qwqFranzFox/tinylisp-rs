use std::collections::HashMap;
use std::rc::Rc;

use crate::types::Data;

fn f_add(a: Data, b: Data, e: &Env) {}

pub struct Env {
    table: HashMap<String, Rc<Data>>,
}

impl Env {
    pub fn get(&self, name: &String) -> Option<&Rc<Data>> {
        return self.table.get(name);
    }
    pub fn set(&mut self, name: String, data: Rc<Data>) {
        self.table.insert(name, data);
    }
}
