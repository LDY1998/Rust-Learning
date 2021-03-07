use repl::Repl;
use lex::Lexer;

mod repl;
mod lex;



fn main() {
    let repl = Repl {
        lexer: Lexer
    };

    repl.run();
}
