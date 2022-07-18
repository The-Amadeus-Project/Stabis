use crate::lexer::Token;
use std::collections::HashMap;

enum ValTypes {
    Int,
    Float,
    String,
    Boolean,
}

struct Checker {
    instructions: Vec<Token>,
    defined_functions: HashMap<String, i32>,
    index: i64,
}
