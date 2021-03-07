use std::{vec::Vec};
use std::iter::Peekable;
use std::num::ParseIntError;

pub struct Lexer;


#[derive(Debug, Clone, PartialEq)]
#[allow(missing_docs)]
pub enum Token {
    Integer(usize),
    Decimal(f64),
    Identifier(String),
    OpenParen,
    CloseParen,
    Plus,
    Minus,
    Times,
    Equals,
}

impl From<usize> for Token {
    fn from(i: usize) -> Token {
        Token::Integer(i)
    }
}

impl From<f64> for Token {
    fn from(f: f64) -> Token {
        Token::Decimal(f)
    }
}

impl From<char> for Token {
    fn from(f: char) -> Token {
        match f {
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '=' => Token::Equals,
            '+' => Token::Plus,
            '-' => Token::Minus,
            _ => panic!("Invalid character token: {}", f),
        }
    }
}

impl From<String> for Token {
    fn from(str: String) -> Token {
        Token::Identifier(str)
    }
}

impl From<&str> for Token {
    fn from(t: &str) -> Token {
        Token::Identifier(t.to_string())
    }
}

impl Lexer {
    pub fn lex(&self, input: &String) -> Result<Vec<Token>, String> {

        let mut res = Vec::new();

        let mut it = input.chars().peekable();

        while let Some(&c) = it.peek() {
            match c {
                '1'..='9' => {
                    res.push(Token::from(Lexer::get_number(c, &mut it)))
                },
                '(' | ')' | '+' | '-' | '=' => {
                    it.next();
                    res.push(Token::from(c))
                },
                'a'..='z' | 'A'..='Z' =>  {
                    let str_token = it.by_ref().take_while(|&c| {
                        c.is_alphanumeric()
                    }).collect::<String>();
                    res.push(Token::from(str_token));
                }
                _ => {
                    it.next();
                }
            }
        }

        println!("Token vector from lexer: {:?}", res);

        Ok(res)
    }

    fn get_number<T: Iterator<Item=char>>(c: char, iter: &mut Peekable<T>) -> usize {
        let mut number = c.to_string().parse::<usize>().expect("Should have pass a integer");

        iter.next();
        while let Some(Ok(digit)) = iter.peek().map(|c| {
            c.to_string().parse::<usize>()
        }) {
            number = number * 10 + digit;
            iter.next();
        }

        number
    }
}

pub fn test_template(input: String, exp: Vec<Token>) {
    let test_lexer = Lexer;        
    let test_input = input;
    let act_res = test_lexer.lex(&test_input).unwrap();
    assert_eq!(act_res, exp);
}
#[cfg(test)]
mod tests {

    use super::*;

    

    #[test]
    fn lex_simple_number() {
        let test_lexer = Lexer;
        let test_input = "12345".to_string();
        assert_eq!(test_lexer.lex(&test_input).unwrap(), vec![Token::Integer(12345)]);
    }

    #[test]
    fn lex_simple_identifier() {
        let test_lexer = Lexer;
        let test_input = "hello".to_string();
        assert_eq!(test_lexer.lex(&test_input).unwrap(), vec![Token::Identifier("hello".to_string())]);
    }

    #[test]
    fn lex_op() {
        test_template(String::from("+"), vec![Token::Plus]);
    }
    
    #[test]
    fn lex_op_number() {
        let test_input = String::from("+ 1 2");
        let exp_res = vec![Token::Plus, Token::Integer(1), Token::Integer(2)];
        test_template(test_input, exp_res);
    }
}