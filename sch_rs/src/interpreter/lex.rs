use std::{vec::Vec};
use std::iter::Peekable;
use std::num::ParseIntError;
use std::fmt;

pub struct SyntaxError {
    msg: String,
}
impl fmt::Debug for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Syntax Error: {}", self.msg)
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Syntax Error: {}", self.msg)
    }
}

macro_rules! syntax_error {
    ($($arg:tt),*) => (
        return Err(SyntaxError { msg: format!($($arg),*)})
    )
}


#[derive(Debug, PartialEq, Clone, Eq)]
#[allow(missing_docs)]
pub enum Token {
    Integer(usize),
    Identifier(String),
    OpenParen,
    CloseParen,
}

impl From<usize> for Token {
    fn from(i: usize) -> Token {
        Token::Integer(i)
    }
}


impl From<char> for Token {
    fn from(f: char) -> Token {
        match f {
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            _ => Token::Identifier(f.to_string()),
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

pub mod lexer {
    use super::*;

    pub fn lex(input: &String) -> Result<Vec<Token>, SyntaxError> {

        let mut res = Vec::new();

        let mut it = input.chars().peekable();


        println!("input: {:?}", it.clone().collect::<String>());
        while let Some(&c) = it.peek() {
            // println!("current char: {}", c);
            match c {
                '1'..='9' => {
                    // println!("current number token: {}", c);
                    // res.push(Token::from(lexer::get_integer(c, &mut it)))
                    let number = get_number_string(c, &mut it);
                    match number.parse::<usize>() {
                        Ok(n) => res.push(Token::from(n)),
                        _ => syntax_error!("only support integer number but got: {:?}", number)
                    }
                },
                '(' | ')' | '+' | '-' | '=' | '[' | ']' => {
                    // println!("current symbol token: {}", it.peek().unwrap());
                    res.push(Token::from(c));
                    it.next();
                },
                'a'..='z' | 'A'..='Z' =>  {
                    // ! we have to clone the iterator, since the it will consume
                    // ! the last item in iterator that the closure returns false
                    // ! we still want to consume the letter going right after identifier
                    let str_token = it.clone().by_ref().take_while(|&c| {
                        let is_iden = c.is_alphanumeric();
                        if is_iden {
                            it.next();
                        }
                        is_iden
                    }).collect::<String>();
                    res.push(Token::from(str_token));
                }
                _ => {
                    // println!("token to skip: {}", it.peek().unwrap());
                    it.next();
                }
            }
        }

        println!("Token vector from lexer: {:?}", res);

        Ok(res)
    }

    fn get_number_string<T: Iterator<Item=char>> (c: char, iter: &mut Peekable<T>) -> String {

        println!("getting the number ...");
        let mut res: String = String::new();
        loop {
            let c = iter.peek().unwrap();
            let digit = c.is_digit(10) || c ==  &'.';
            if digit {
                res.push(*c);
                iter.next();
            } else {
                break;
            }
        }

        res
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    

    #[test]
    fn lex_simple_number() {
        let test_input = "12345".to_string();
        assert_eq!(lexer::lex(&test_input).unwrap(), vec![Token::Integer(12345)]);
    }

    #[test]
    fn lex_simple_identifier() {
        let test_input = "hello".to_string();
        assert_eq!(lexer::lex(&test_input).unwrap(), vec![Token::Identifier("hello".to_string())]);
    }
    

}