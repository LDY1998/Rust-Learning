use crate::interpreter::lex::{Lexer, Token};
use crate::interpreter::parser::*;

pub fn parse_test_template(input: Vec<Token>, exp: Vec<Node>) {

    let act = Parser::parse(&input).unwrap();
    assert_eq!(act, exp);

}
pub fn lex_test_template(input: String, exp: Vec<Token>) {
    let test_input = input;
    let act_res = Lexer::lex(&test_input).unwrap();
    assert_eq!(act_res, exp);
}