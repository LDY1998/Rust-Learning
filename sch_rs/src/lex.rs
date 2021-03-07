use std::{vec::Vec};
use std::iter::Peekable;

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

impl Lexer {
    pub fn lex(&self, input: &String) -> Result<Vec<Token>, String> {

        let mut res = Vec::new();

        let mut it = input.chars().peekable();

        while let Some(&c) = it.peek() {
            match c {
                '0'..='9' => {
                    res.push(Token::from(Lexer::get_number(c, &mut it)))
                },
                '(' | ')' | '+' | '-' | '=' => {
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

        while let Some(Ok(digit)) = iter.peek().map(|c| c.to_string().parse::<usize>()) {
            number = number * 10 + digit;
            iter.next();
        }

        number
    }
}