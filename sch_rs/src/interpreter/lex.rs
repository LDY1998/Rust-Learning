use std::{vec::Vec};
use std::iter::Peekable;
use std::num::ParseIntError;



#[derive(Debug, PartialEq, Clone, Hash, Eq)]
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

    pub fn lex(input: &String) -> Result<Vec<Token>, String> {

        let mut res = Vec::new();

        let mut it = input.chars().peekable();


        println!("input: {:?}", it.clone().collect::<String>());
        while let Some(&c) = it.peek() {
            // println!("current char: {}", c);
            match c {
                '1'..='9' => {
                    // println!("current number token: {}", c);
                    res.push(Token::from(lexer::get_number(c, &mut it)))
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