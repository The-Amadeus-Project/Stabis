use std::collections::HashMap;
use crate::lexer::Token;

enum ValTypes {
    Int,
    Float,
    String,
    Boolean
}

struct Checker {
    instructions: Vec<Token>,
    defined_functions: HashMap<String, i32>,
    index: i64
}