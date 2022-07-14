use std::env::args;
use std::process::exit;
use crate::sb_is::{compile, interpret};
mod lexer;
mod sb_is;
mod interpreter;
// mod check;

fn usage(){
    let message = r#"------------USAGE------------
command filepath [mode]

[modes] [params]        [decs]
    -i                   interpret program
    -c  <output_path>    compile program
    "#;
    println!("{}", message);
}

fn main() {
    let mut args: Vec<String> = args().collect();
    if args.len() == 2 {
        usage();
        exit(1)
    }
    let _program_path = args.remove(0);
    let file_path = args.remove(0);
    let mode = args.remove(0);
    if mode == "-c" {
        let file_content = std::fs::read_to_string(file_path).unwrap();
        let result = compile(file_content);
    } else if mode == "-i" {
        interpret("main.sbis".to_string(), true)
    } else {
        panic!("LOL NO!")
    }
}
