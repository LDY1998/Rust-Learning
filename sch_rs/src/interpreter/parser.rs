use super::lex::{Token};
use std::slice;
use crate::util::parse_test_template;


#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub enum Node {
    Identifier(String),
    Integer(usize),
    List(Vec<Node>),
}

pub struct Parser<'a> {
    tokens: slice::Iter<'a, Token>
}

impl<'a> Parser<'a> {
    pub fn parse(tokens: &Vec<Token>) -> Result<Vec<Node>, String> {

        let mut parser = Parser {
            tokens: tokens.iter(),
        };

        parser.parse_nodes(0)
    }

    fn parse_nodes(&mut self, depth: u32) -> Result<Vec<Node>, String> {
        let mut nodes = Vec::new();
        loop {
            match self.parse_node(depth) {
                Ok(Some(node)) => {nodes.push(node);},
                Ok(None) =>  {
                    return Ok(nodes);
                },
                _ => {
                    return Err("Error in parsing nodes: {}".to_string());
                }
            }
        }
    }
    fn parse_node(&mut self, depth: u32) -> Result<Option<Node>, String> {

        match self.tokens.next() {
            Some(token) => {
                match token {
                    Token::Integer(i) => Ok(Some(Node::Integer(i.clone()))),
                    Token::OpenParen => {
                        let inner = self.parse_nodes(depth+1).unwrap();
                        Ok(Some(Node::List(inner)))
                    },

                    Token::CloseParen => {
                        if depth > 0 {
                            Ok(None)
                        } else {
                            Err("No matching open paren!".to_string())
                        }
                    },

                    Token::Identifier(name) => Ok(Some(Node::Identifier(name.to_string()))),

                }
            },
            None => {
                if depth == 0 {
                    Ok(None)
                } else {
                    Err(format!("Unexpected end of input at depth: {}", depth))
                }
            },
            _ => Err("Invalid token type!".to_string()),
        }
    }
   
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_int() {
        let input = vec![Token::Integer(1234)];
        let exp = vec![Node::Integer(1234)];
        parse_test_template(input, exp);
    }
    
    #[test]
    fn parse_paren_int() {
        let input = vec![Token::OpenParen, Token::Integer(1234), Token::CloseParen];
        let exp = vec![Node::List(vec![Node::Integer(1234)])];
        parse_test_template(input, exp);
    }
}



