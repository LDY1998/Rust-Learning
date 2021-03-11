use fmt::Display;

use super::parser::Node;
use std::{cell::{RefCell}, collections::HashMap, rc::Rc};
use std::fmt;




pub struct RuntimeError {
    msg: String,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runtime Error: {}", self.msg)
    }
}

impl fmt::Debug for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Runtime Error: {}", self.msg)
    }
}

macro_rules! runtime_error {
    ($($arg:tt),*) => (
        return Err(RuntimeError { msg: format!($($arg),*)})
    )
}

// macro_rules! runtime_error_1 {
//     ((arg:tt)*) => (
//         return Err(RuntimeError { msg: format!((arg)*)})
//     )
// }


#[derive(PartialEq, Clone)]
pub enum Value {
    Unit,
    Symbol(String),
    Integer(usize),
    List(Vec<Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Unit => write!(f, "()"),
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

impl Value {

    fn from_nodes(nodes: &[Node]) -> Vec<Value> {
        nodes.iter().map(Value::from_node).collect()
    }

    fn from_node(node: &Node) -> Value {
        match node {
            Node::Identifier(s) => Value::Symbol(s.clone()),
            Node::Integer(i) => Value::Integer(i.clone()),
            Node::List(nodes) => Value::List(Value::from_nodes(nodes)),
        }
    }
}

/*
`(define x 2)
`(+ x x x)
*/

// [Node::List
//     (vec![
//         Node::Identifier("define".to_string()), 
//         Node::Identifier("x".to_string()), 
//         Node::Integer(2)]), 
//         Node::List(vec![
//             Node::Identifier("+".to_string()), 
//             Node::Identifier("x".to_string()), 
//             Node::Identifier("x".to_string()), 
//             Node::Identifier("x".to_string())
//         ])]

#[derive(Clone)]
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    values: HashMap<String , Value>,
}

impl Env {


    fn new_root(&self) -> Rc<RefCell<Env>> {
        Rc::new(RefCell::new(Env {
            parent: None,
            values: HashMap::new(),
        }))
    }

    fn get_root(env_ref: Rc<RefCell<Env>>) -> Rc<RefCell<Env>> {
        let env = env_ref.borrow();
        match env.parent {
            Some(ref p) => Env::get_root(p.clone()),
            None => env_ref.clone(),
        }
    }
    fn new_child(env: Rc<RefCell<Env>>) -> Rc<RefCell<Env>> {
        let new_env = Env {
            parent: Some(env),
            values: HashMap::new(),
        };

        Rc::new(RefCell::new(new_env))
    }

    pub fn define(&mut self, key: &String, value: &Value) -> Result<(), RuntimeError> {
        match self.values.insert(String::from(key), value.clone()) {
            Some(_) => runtime_error!("The identifier is already defined!: {:?}", key),
            None => Ok(()),
        }
    }

    pub fn set(&mut self, key: &String, value: &Value) -> Result<(), RuntimeError> {
        match self.values.contains_key(key) {
            true => {
                self.values.insert(key.clone(), value.clone());
                Ok(())
            },
            false => {
                match &self.parent {
                    Some(p) => p.borrow_mut().set(key, value),
                    None => runtime_error!("Can't set an undefined variable: {:?}", key)
                }
            }
        }
    }

    pub fn get(&self, identifier: &String) -> Result<Value, RuntimeError> {
        match self.values.get(identifier) {
            Some(v) => Ok(v.clone()),
            None => {
                match &self.parent {
                    Some(p) => p.borrow().get(identifier),
                    None => runtime_error!("Used before define: {:?}", identifier),
                }
            },
        }
    }


}


/*
   TODO: The public eval function to produce a value based on AST
*/
pub fn eval(nodes: &Vec<Node>, env: Rc<RefCell<Env>>) -> Result<Value, RuntimeError> {
    let values = Value::from_nodes(nodes);

    eval_values(&values, env)
}

fn eval_values(values: &Vec<Value>, env: Rc<RefCell<Env>>) -> Result<Value, RuntimeError> {
    let mut res = None;
    for v in values {
        res = Some(eval_value(&v, env.clone()).unwrap());
    }

    match &res {
        Some(val) => Ok(val.clone()),
        None => Ok(Value::Unit)
    }
}

fn eval_value(value: &Value, env: Rc<RefCell<Env>>) -> Result<Value, RuntimeError> {

    match value {
        Value::Unit => Ok(value.clone()),
        Value::Integer(i) => Ok(value.clone()),
        Value::Symbol(s) => env.borrow().get(s),
        Value::List(_) => Ok(value.clone()),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // * define a list of variables in current level for testing

    fn insert_into_env(env: Rc<RefCell<Env>>, vars: &Vec<(String, Value)>) -> Rc<RefCell<Env>> {

        for (key, value) in vars {
            env.borrow_mut().define(key, value);
        }

        env
    }


    fn test_template(nodes: Vec<Node>, exp: Value) {
        
    }
}
