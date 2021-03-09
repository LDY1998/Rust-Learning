use super::parser::Node;




struct RuntimeError;


pub enum Value {
    Symbol(String),
    Integer(i32),
    List(Vec<Value>),
}

pub fn eval(nodes: &Vec<Node>) -> Result<Value, RuntimeError> {


}