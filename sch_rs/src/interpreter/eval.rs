use fmt::Display;

use super::parser::Node;
use std::{cell::{RefCell}, collections::HashMap, rc::Rc};
use std::fmt;



struct RuntimeError;


pub enum Value {
    Symbol(String),
    Integer(i32),
    List(Vec<Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Symbol(s) => write!(f, "{}", s),
            Value::Integer(i) => write!(f, "{}", i),
            Value::List(values) => {
                let strs: Vec<String> = values.iter().map(|v| {
                    format!("{}", v)
                }).collect();

                write!(f, "({})", &strs.join(" "))
            }
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Symbol(s) => write!(f, "{}", s),
            Value::Integer(i) => write!(f, "{}", i),
            Value::List(values) => {
                let strs: Vec<String> = values.iter().map(|v| {
                    format!("{:?}", v)
                }).collect();

                write!(f, "({})", &strs.join(" "))
            }
            _ => write!(f, "{}", self)
        }
    }
}

pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    values: HashMap<String , Value>,
}

impl Env {
    fn new_child(env: Rc<RefCell<Env>>) -> Rc<RefCell<Env>> {
        let new_env = Env {
            parent: Some(env),
            values: HashMap::new(),
        };

        Rc::new(RefCell::new(new_env))
    }
}


/*
   TODO: The public eval function to produce a value based on AST
*/
// pub fn eval(nodes: &Vec<Node>) -> Result<Value, RuntimeError> {


// }