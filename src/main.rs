use crate::sb_is::{compile, interpret};
use std::env::args;
use std::io;
use std::io::Write;
use std::process::{exit, Command};
mod base;
mod compiler;
mod interpreter;
mod lexer;
mod sb_is;
mod type_checker;
// mod check;

fn usage() {
    let message = r#"------------USAGE------------
command filepath [mode] [etc]

[modes] [params]        [decs]
    -i                   interpret program
    -c                   compile program

[etc]
    --debug             debug, shows tokenss

    "#;
    println!("{}", message);
}

fn main() {
    let mut args: Vec<String> = args().collect();
    if args.len() < 3 {
        usage();
        exit(1)
    }
    let _program_path = args.remove(0);
    let file_path = args.remove(0);
    let mode = args.remove(0);
    let mut debug = false;
    for arg in args {
        if arg == "--debug" {
            debug = true;
        }
    }
    if mode == "-c" {
        let result = compile(file_path.clone(), debug);
        let mut file_res = std::fs::File::create(file_path.replace(".sbis", ".rs"));
        if file_res.is_err() {
            std::fs::write(file_path.replace(".sbis", ".rs"), result).unwrap();
        } else {
            let mut file = file_res.unwrap();
            let res = file.write_all(result.as_ref());
        }
        Command::new("rustc")
            .args([
                &*file_path.replace(".sbis", ".rs"),
                "-o",
                &*file_path.replace(".sbis", ""),
            ])
            .output()
            .expect("failed to execute process");
    } else if mode == "-r" {
        let result = compile(file_path.clone(), debug);
        let mut file_res = std::fs::File::create(file_path.replace(".sbis", ".rs"));
        if file_res.is_err() {
            std::fs::write(file_path.replace(".sbis", ".rs"), result).unwrap();
        } else {
            let mut file = file_res.unwrap();
            let res = file.write_all(result.as_ref());
        }
        Command::new("rustc")
            .args([
                &*file_path.replace(".sbis", ".rs"),
                "-o",
                &*file_path.replace(".sbis", ""),
                "-q",
            ])
            .output()
            .expect("failed to execute process");
    } else if mode == "-i" {
        interpret(file_path, debug)
    } else {
        panic!("LOL NO!")
    }
}
