use crate::interpreter::Interpreter;
use crate::lexer::{Lexer, Token, TokenType };

fn lex(file_path: String, debug: bool) -> Vec<Token> {
    let file_content = std::fs::read_to_string(file_path).expect("couldnt open file");
    let mut lexed = Lexer::new().lex_text(file_content);
    if debug {
        for part in &lexed {
            println!("{:?}", part)
        }
        println!("--------------------------------------------------------");
    }
    // let mut type_checker = Checker::new();
    // type_checker.check_program(return_parsed.clone());
    lexed
}

// fn pre_compile(file_path: String, debug: bool) -> Parsed {
//     let file_content = std::fs::read_to_string(file_path).expect("couldnt open file");
//     let mut the_parser = parser::Parser::new(debug);
//     let return_parsed = the_parser.parse_text(file_content);
//     if debug {
//         if let Parsed::Program(parsed) = &return_parsed {
//             for part in parsed {
//                 println!("{:?}", part)
//             }
//         }
//         println!("--------------------------------------------------------");
//     }
//     // let mut type_checker = Checker::new();
//     // type_checker.check_program(return_parsed.clone());
//     return_parsed
// }


pub fn compile(file_path: String) {
    unimplemented!()
}

pub fn interpret(file_path: String, debug: bool) {
    let lexed = lex(file_path, debug);
    Interpreter::new(lexed).run();
}