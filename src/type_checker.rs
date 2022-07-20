use crate::lexer::Token;
use std::collections::HashMap;

enum ValTypes {
    Int,
    Float,
    String,
    Boolean,
    Any
}

struct Checker {
    stacks: HashMap<usize, Vec<ValTypes>>,
    instructions: Vec<Token>,
    defined_functions: HashMap<String, (Vec<ValTypes>, Vec<ValTypes>)>,
    index: i64,
}

impl Checker {
    fn new(){

    }
}
