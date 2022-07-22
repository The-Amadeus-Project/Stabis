use crate::lexer::{Token, TokenType};
use std::collections::HashMap;
use crate::base::Value;

#[derive(Debug, Clone, PartialEq)]
enum ValTypes {
    Int,
    Float,
    Str,
    Bool,
    Any
}


pub struct Checker {
    stacks: HashMap<usize, Vec<ValTypes>>,
    instructions: Vec<Token>,
    last_ints: Vec<i128>, // for the create_stack
    defined_functions: HashMap<String, (Vec<ValTypes>, Vec<ValTypes>)>,
    current_instruction: Token,
    index: usize,
}

impl Checker {
    pub fn new(instructions: Vec<Token>) -> Self {
        let mut new = Self {
            stacks: HashMap::new(),
            instructions,
            current_instruction: Token {
                token_type: TokenType::NullForParser,
                value: "".to_string(),
                x: 0,
                y: 0
            },
            last_ints: vec![],
            defined_functions: HashMap::new(),
            index: 0,
        };
        let mut funcs: HashMap<String, (Vec<ValTypes>, Vec<ValTypes>)> = HashMap::new();
        // funcs.insert("".to_string(), (vec![], vec![])); template
        funcs.insert("print".to_string(), (vec![ValTypes::Any], vec![]));
        funcs.insert("push".to_string(), (vec![ValTypes::Int, ValTypes::Any], vec![]));
        funcs.insert("pop".to_string(), (vec![ValTypes::Int], vec![ValTypes::Any]));
        funcs.insert("create_stack".to_string(), (vec![ValTypes::Int], vec![]));

        new.defined_functions = funcs;
        new.stacks.insert(0, vec![]);
        new
    }
    fn next_token(&mut self) -> bool{
        self.index += 1;
        if self.index == self.instructions.len(){
            false
        } else {
            self.current_instruction = self.instructions[self.index].clone();
            true
        }
    }
    fn error(&mut self, error_body: String, pos_x: u32, pos_y: u32, call_name: String) {
        panic!(
            "At line {} char {},\n  Couldn't execute '{}' because of\n  '{}'  ",
            pos_y, pos_x, call_name, error_body
        )
    }
    fn check(&mut self, stack: usize, nums: u32, for_main: bool, name: String, pos_x: u32, pos_y: u32, ) {
        if !self.stacks.contains_key(&stack) {
            let vals = &self.stacks.values();
            self.error(format!("Error: stack {} doesnt exists, only stacks {:?} exists'", stack, vals), pos_x, pos_y, name, )
        } else if stack < 1 && !for_main {
            self.error(format!("Error: accessing special stack {} use different command", stack),pos_x,pos_y, name, )
        } else if self.stacks.get(&stack).unwrap().len() < nums as usize {
            self.error(format!("Error: stack {} doesnt have enough items expected {} got {}", stack, nums, self.stacks.get(&stack).unwrap().len()),pos_x,pos_y, name,)
        }
    }
    pub fn _push(&mut self, stack: usize, value: ValTypes, pos_x: u32, pos_y: u32) {
        self.check(stack, 0, false, "push".to_string(), pos_x, pos_y);
        self.stacks.get_mut(&stack).unwrap().push(value);
    }
    pub fn _pop(&mut self, stack: usize, pos_x: u32, pos_y: u32) {
        self.check(stack, 1, false, "pop".to_string(), pos_x, pos_y);
        let popped = self.stacks.get_mut(&stack).unwrap().pop().unwrap();
        self.push_main(popped);
    }
    pub fn _drop(&mut self, pos_x: u32, pos_y: u32) {
        self.check(0, 1, true, "drop".to_string(), pos_x, pos_y);
        self.pop_main();
    }
    pub fn push_main(&mut self, value: ValTypes) {
        self.stacks.get_mut(&0).unwrap().push(value);
    }
    pub fn pop_main(&mut self) -> ValTypes {
        self.stacks.get_mut(&0).unwrap().pop().unwrap()
    }
    pub fn _create_stack(&mut self, stack: usize, pos_x: u32, pos_y: u32) {
        let name = "create_stack";
        if self.stacks.contains_key(&stack) {
            self.error(
                format!("Error: stack {} exists", stack),
                pos_x,
                pos_y,
                name.to_string(),
            )
        } else if stack < 1 {
            self.error(
                format!(
                    "Error: creating a special stack is not allowed : stack {}",
                    stack
                ),
                pos_x,
                pos_y,
                name.to_string(),
            )
        }
        self.stacks.insert(stack, vec![]);
    }
    fn end(&mut self){
        let mut error = false;
        for stack in &self.stacks {
            if stack.1.len() > 0 {
                println!("Error! Unhandled data on stack {} : {:?}", stack.0, stack.1);
                error = true
            }
        }

        if error {
            panic!()
        }
    }
    fn built_in(&mut self){
        match &*self.current_instruction.value {
            "push" => {
                self.check(0, 2, true, "push".to_string(), self.current_instruction.x, self.current_instruction.y);
                let val_to_push = self.pop_main();
                if val_to_push == ValTypes::Int {
                    self.last_ints.pop().unwrap();
                }
                let stack_to_push_type = self.pop_main();
                if stack_to_push_type != ValTypes::Int {
                    self.error(
                        format!(
                            "Error: Expected push stack to be an Int got {:?}",
                            stack_to_push_type
                        ),
                        self.current_instruction.x,
                        self.current_instruction.y,
                        "push".to_string(),
                    )
                }
                let stack_to_push = self.last_ints.pop().unwrap();
                self.check(0, 0, true, "push".to_string(), self.current_instruction.x, self.current_instruction.y);
                self._push(stack_to_push as usize, val_to_push, self.current_instruction.x, self.current_instruction.y)
            }
            "print" => {
                self.check(0, 1, true, "push".to_string(), self.current_instruction.x, self.current_instruction.y);
                self.pop_main();
            }
            _ => {unimplemented!("-?> '{}'", self.current_instruction.value)}
        }
        unimplemented!("-> {:?}", self.current_instruction)
    }
    pub fn run(&mut self){
        self.current_instruction = self.instructions[self.index].clone();
        loop {
            if self.current_instruction.is_data_type(){
                match self.current_instruction.token_type {
                    TokenType::Integer => {
                        self.stacks.get_mut(&0).unwrap().push(ValTypes::Int);
                        self.last_ints.push(self.current_instruction.value.parse::<i128>().unwrap())
                    }
                    TokenType::String => {self.stacks.get_mut(&0).unwrap().push(ValTypes::Str)}
                    TokenType::Boolean => {self.stacks.get_mut(&0).unwrap().push(ValTypes::Bool)}
                    TokenType::FloatingPoint => {self.stacks.get_mut(&0).unwrap().push(ValTypes::Float)}
                    _ => unimplemented!("Unexpected")
                }
            } else {
                match self.current_instruction.token_type {
                    TokenType::Identifier => {
                        if self.defined_functions.contains_key(&self.current_instruction.value){
                            self.built_in()
                        } else {
                            unimplemented!("-> {:?}", self.current_instruction)
                        }
                    }
                    TokenType::EndOfFile => {
                        break
                    }
                    _ => {
                        unimplemented!("-> {:?}", self.current_instruction)
                    }
                }
            }



            if !self.next_token(){
                panic!("unexpected end")
            }
        }
        self.end()
    }
    pub fn check_instructions(instructions: Vec<Token>) {
        Checker::new(instructions).run()
    }
}
