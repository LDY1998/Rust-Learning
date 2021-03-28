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


#[derive(PartialEq, Clone)]
pub enum Value {
    Unit,
    Symbol(String),
    Integer(usize),
    List(Vec<Value>),
    Procedure(Function),
}

pub type ValueOperation = fn(&[Value], Rc<RefCell<Env>>) -> Result<Value, RuntimeError>;

pub enum Function {
    Native(ValueOperation),
    Closure(Vec<String>, Vec<Value>, Rc<RefCell<Env>>),
}

impl PartialEq for Function{
    fn eq(&self, other: &Function) -> bool {
        self == other
    }
}

/** 
 * * evaluate expression, if the first item is a procedure we apply it
 * * otherwise we just process it with eval_values
 */
fn eval_expression(vals: &[Value], env: Rc<RefCell<Env>>) -> Result<Value, RuntimeError> {
    let p = eval_value(&vals[0], env.clone()).unwrap();

    match &p {
        Value::Procedure(f) => native_apply(f.clone(), &vals[1..], env),
        _ => runtime_error!("first entry must be procedure: {:?}", vals),
    }
}

/**
 * * (lambda (xs ...) body) produce a procedure
 */
fn native_lambda(args: &[Value], env: Rc<RefCell<Env>>) -> Result<Value, RuntimeError> {
    let params = match &args[0] {
        Value::List(ns) => {

            let params: Result<Vec<String>, RuntimeError> = ns.iter().map(|nv| {
                match nv {
                    Value::Symbol(s) => Ok(s.to_string()),
                    _ => runtime_error!("Must provide symbol as parameter names: {:?}", nv),
                }
            }).collect();

            params.unwrap()
        }
        _ => runtime_error!("Must provide parameter lists in function parameter: {:?}", args),
    };

    Ok(Value::Procedure(Function::Closure(params, args[1..].to_vec(), Env::new_child(env.clone()))))
}

/** 
 * *(p_name arg1 arg2 ...) evaluate the function body after all args evalualted in current env
 * ! note that we only evaluate the argument in current env when a closure is applied
 * ! native function don't require the argument evaluation, they should already be a valid value
*/
fn native_apply(func: Function, apply_args: &[Value], env: Rc<RefCell<Env>>) -> Result<Value, RuntimeError> {
    // let args = apply_args[..].to_vec();
    // let eval_args: Vec<Value> = args.iter().map(|a| {
    //     eval_value(a, env.clone()).unwrap()
    // }).collect();
    match &func {
        Function::Native(op) => {
            op(&apply_args[..], env)
        },
        Function::Closure(params, body, closure_env) => {
            let closure_env = closure_env.clone();
            let new_env = Env::new_child(closure_env);
            for (param, arg) in params.iter().zip(apply_args.iter()) {
                new_env.borrow_mut().define(param, &eval_value(&arg, env.clone()).unwrap()).unwrap();
            }

            eval_values(body, new_env)
        }

        _ => runtime_error!("expect procedure apply but got: {:?}", apply_args),
    }
} 

/** 
 * * (let ([n1 v1] ...) body)
*/
fn native_let(args: &[Value], env: Rc<RefCell<Env>>) -> Result<Value, RuntimeError> {
    let new_env = Env::new_child(env.clone());
    let eval_nv: Result<(), RuntimeError> = match &args[0] {
        Value::List(assigns) => {
            for assign in assigns {
                match assign {
                    Value::List(nv_pair) => {
                        assert!(nv_pair.len() == 2);
                        match &nv_pair[0] {
                            Value::Symbol(ref s) => {
                                new_env.borrow_mut().define(s, &eval_value(&nv_pair[1], env.clone()).unwrap());
                            },
                            _ => runtime_error!("invalid let syntax: {:?}", nv_pair),
                        }
                    },
                    _ => runtime_error!("invalid let define list: {:?}", assign)
                }
            }
            Ok(())
        },
        _ => runtime_error!("let-define requires but got: {:?}", args),
    };

    eval_nv.unwrap();

    eval_value(&args[1], new_env)

}

/**
 * * a general arithmatic native function for arithmatic operation
 * ! args must be all Value::Integer, otherwise an runtime error is reported
 */
fn native_arithmatic(args: &[Value], env: Rc<RefCell<Env>>, f: fn(i1: usize, i2: usize) -> usize) -> Result<Value, RuntimeError> {
    let args: Result<Vec<usize>, RuntimeError> = args.iter().map(|x| {
        let v = eval_value(x, env.clone()).unwrap();
        match v {
            Value::Integer(i) => Ok(i),
            _ => runtime_error!("invalid arguments for add: {:?}", v),
        }
    }).collect();

    //  ! we want to do arithmatic with arg[0] as initial and go over the vec
    //  ! we need to advance the iterator one step so that we do that
    //  ! the old way is directly call fold with args[0] as initial and that will compute args[0] twice
    let args = args.unwrap();
    let mut args_it = args.iter();
    args_it.next();
    let res = args_it.fold(args[0], |acc, x| {
        f(acc, *x)
    });

    Ok(Value::Integer(res))

}

fn native_add(args: &[Value], env: Rc<RefCell<Env>>) -> Result<Value, RuntimeError> {
    native_arithmatic(args, env, |a, b| {
        a + b
    })
}

fn native_times(args: &[Value], env: Rc<RefCell<Env>>) -> Result<Value, RuntimeError> {
    native_arithmatic(args, env, |a, b| {
        a * b
    })
}
/*
 * * (define name value)\(define (p_name params) body)
 * args must be a vec with length greater than 2
 */
fn native_define(args: &[Value], env: Rc<RefCell<Env>>) -> Result<Value, RuntimeError> {
    assert!(args.len() >= 2);

    let (name, val) = match &args[0] {
        Value::Symbol(n) => {
            let val = eval_value(&args[1], env.clone()).unwrap();
            (n, val)
        }
        Value::List(list) => {
            assert!(list.len() >= 2);
            match &list[0] {
                Value::Symbol(n) => {
                    let res: Result<Vec<String>, RuntimeError> = (&list[1..]).iter().map(|x| {
                        match x {
                            Value::Symbol(s) => Ok(s.clone()),
                            _ => runtime_error!("unexpected argument in define: {:?}", x),
                        }
                    }).collect();

                    let params = res.unwrap();
                    let body = (&args[1..]).to_vec();
                    let val = Value::Procedure(Function::Closure(params, body, env.clone()));
                    (n, val)
                },
                _ => runtime_error!("must supply a symbol as define name: {:?}", list),
            }
        },
        _ => runtime_error!("invalid define: {:?}", args),
    };

    env.borrow_mut().define(&name, &val).unwrap();
    Ok(val)
}

impl Clone for Function {
    fn clone(&self) -> Function {
        // self.clone()
        match self {
            Function::Native(op) => Function::Native(op.clone()),
            Function::Closure(params, body, env) => Function::Closure(params.clone(), body.clone(), env.clone()),
        }
    }
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
            },
            Value::Procedure(_) => {
                write!(f, "#procedure")
            },
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
    values: HashMap<String, Value>,
}

impl Env {


    pub fn new_root() -> Rc<RefCell<Env>> {
       let mut env =  Env {
           parent: None,
           values: HashMap::new(),
       };

       env.define("define", &Value::Procedure(Function::Native(native_define))).unwrap();
       env.define("+", &Value::Procedure(Function::Native(native_add))).unwrap();
       env.define("let", &Value::Procedure(Function::Native(native_let))).unwrap();
       env.define("*", &Value::Procedure(Function::Native(native_times))).unwrap();
       env.define("lambda", &Value::Procedure(Function::Native(native_lambda))).unwrap();

        Rc::new(RefCell::new(env))
    }

    fn get_root(env_ref: Rc<RefCell<Env>>) -> Rc<RefCell<Env>> {
        let env = env_ref.borrow();
        match env.parent {
            Some(ref p) => Env::get_root(p.clone()),
            None => env_ref.clone(),
        }
    }
    // * return the new child env rc with parameter as its parent
    pub fn new_child(env: Rc<RefCell<Env>>) -> Rc<RefCell<Env>> {
        let new_env = Env {
            parent: Some(env),
            values: HashMap::new(),
        };

        Rc::new(RefCell::new(new_env))
    }

    fn define_internal(&mut self, key: &String, value: &Value) -> Result<(), RuntimeError> {
        match self.values.insert(String::from(key), value.clone()) {
            Some(_) => runtime_error!("The identifier is already defined!: {:?}", key),
            None => Ok(()),
        }
    }

    pub fn define(&mut self, key: &str, value: &Value) -> Result<(), RuntimeError> {
        self.define_internal(&key.to_string(), value)
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

pub struct Evalator {
    root: Rc<RefCell<Env>>
}

impl Evalator {
    pub fn new() -> Evalator {
        Evalator {
            root: Env::new_root(),
        }
    }

    pub fn eval(&self, nodes: &Vec<Node>) -> Result<Value, RuntimeError> {
        eval(nodes, self.root.clone())
    }
}


/*
   TODO: The public eval function to produce a value based on AST
*/
fn eval(nodes: &Vec<Node>, env: Rc<RefCell<Env>>) -> Result<Value, RuntimeError> {
    let values = Value::from_nodes(nodes);
    println!("values from nodes: {:?}", values);
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
        Value::List(vs) => {
            eval_expression(vs, env.clone())
        },
        Value::Procedure(_) => Ok(value.clone()),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /**
     * * define a list of variables in current level env
     * ! only for testing
     */

    fn insert_into_env(env: Rc<RefCell<Env>>, vars: &Vec<(String, Value)>) -> Rc<RefCell<Env>> {

        for (key, value) in vars {
            env.borrow_mut().define(key, value);
        }

        env
    }

    fn slice_to_node(idens: Vec<&str>) -> Vec<Node> {
        let nodes = idens.iter().map(|s| {
            Node::Identifier(s.to_string())
        }).collect::<Vec<Node>>();

        nodes
    }

    fn test_template(nodes: Vec<Node>, exp: Value, env: Rc<RefCell<Env>>) {

        match eval(&nodes, env) {
            Ok(v) => assert_eq!(v, exp),
            _ => eprintln!("Test failure: evaluating: {:?}, expect: {:?}", nodes, exp),
        }
    }

    #[test]
    fn eval_simple_integer() {
        test_template(vec![Node::Integer(1)], Value::Integer(1), Env::new_root());
    }

    #[test]
    fn eval_simple_iden() {
        test_template(vec![Node::Identifier("x".to_string())], Value::Integer(1), insert_into_env(Env::new_root(), &vec![("x".to_string(), Value::Integer(1))]));
    }


    // * (define x 2) x
    #[test]
    fn eval_define() {
        let def_nodes = vec![Node::List(vec![Node::Identifier("define".to_string()), Node::Identifier("x".to_string()), Node::Integer(2)]), Node::Identifier("x".to_string())];
        test_template(def_nodes, Value::Integer(2), Env::new_root());
    }

    // * (let ([x 2]) x)
    #[test]
    fn eval_let() {
        let nodes = vec![Node::List(vec![Node::Identifier("let".to_string()), Node::List(vec![Node::List(vec![Node::Identifier("x".to_string()), Node::Integer(2)])]), Node::Identifier("x".to_string())])];
        test_template(nodes, Value::Integer(2), Env::new_root());
    }


}
