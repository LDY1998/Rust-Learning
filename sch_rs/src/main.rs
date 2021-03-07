use sch_rs::repl::Repl;
use sch_rs::lex::Lexer;



fn main() {
    let repl = Repl {
        lexer: Lexer
    };

    repl.run();
}
