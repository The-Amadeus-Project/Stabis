use crate::sb_is::interpret;
mod lexer;
mod sb_is;
mod interpreter;
// mod check;

fn main() {
    interpret("main.sbis".to_string(), true)
}
