use crate::compiler::Compiler;
use crate::interpreter::Interpreter;
use crate::lexer::{Lexer, Token};
use crate::checker::Checker;

fn lex(file_path: String, debug: bool) -> Vec<Token> {
    let file_content = std::fs::read_to_string(file_path).expect("couldnt open file");
    let lexed = Lexer::new().lex_text(file_content);
    if debug {
        for part in &lexed {
            println!("{:?}", part)
        }
        println!("--------------------------------------------------------");
    }
    lexed
}

fn pre_compile(file_path: String, debug: bool) -> Vec<Token> {
    let returned_lexed = lex(file_path, debug);
    Checker::check_instructions(returned_lexed.clone());
    returned_lexed
}

pub fn compile(file_path: String, debug: bool) -> String {
    let lexed = pre_compile(file_path, debug);
    Compiler::new(lexed).run()
}

pub fn interpret(file_path: String, debug: bool) {
    let lexed = lex(file_path, debug);
    // let lexed = pre_compile(file_path, debug);
    Interpreter::new(lexed).run();
}
