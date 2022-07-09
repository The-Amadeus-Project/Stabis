use std::collections::HashMap;
use crate::lexer::{Token, TokenType};

#[derive(PartialEq, Eq, Debug, Clone)]
enum ValueType {
    Int,
    Float
}

#[derive(PartialEq, Debug, Clone)]
struct Value {
    val_type: ValueType,
    val_int: i128,
    val_float: f64
}

impl Value {
    fn new_int(value: i128) -> Self {
        Self {
            val_type: ValueType::Int,
            val_int: value,
            val_float: 0.0
        }
    }
    fn new_float(value: f64) -> Self {
        Self {
            val_type: ValueType::Float,
            val_int: 0,
            val_float: value
        }
    }
    // fn new_int(value) -> {
    //
    // }
}

pub(crate) struct Interpreter {
    instructions: Vec<Token>,
    built_in_functions: Vec<String>,
    stacks: HashMap<usize, Vec<Value>>
}

impl Interpreter {
    fn check(&mut self, stack: usize, nums: u32, name: String, pos_x: u32, pos_y: u32){
        if !self.stacks.contains_key(&stack){
            let vals = &self.stacks.values();
            panic!("At line {} char {}, Couldn't execute '{}' because of 'Error: stack {} doesnt exists, only stacks {:?} exists'",  pos_y, pos_x,name, stack, vals)
        } else if stack < 1 {
            panic!("At line {} char {}, Couldn't execute '{}' because of 'Error: accessing special stack {} use different command'",  pos_y, pos_x, name, stack)
        } else if self.stacks.get(&stack).unwrap().len() < nums as usize {
            panic!("At line {} char {}, Couldn't execute '{}' because of 'Error: stack {} doesnt have enough items expected {} got {}'",  pos_y, pos_x, name, stack, nums, self.stacks.get(&stack).unwrap().len())
        }
    }
    // // fn get_nums(&mut self, stack: usize, amount_of_nums: u32) -> Vec<i128>{
    // //     let mut got_nums = vec![];
    // //     for _ in 0..amount_of_nums {
    // //         got_nums.push(self.pop(stack))
    // //     }
    // //     got_nums
    // // }
    pub fn end(&mut self) {
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
    pub fn push(&mut self, stack: usize, value: Value, pos_x: u32, pos_y: u32){
        self.check(stack, 0, "push".to_string(), pos_x, pos_y);
        self.stacks.get_mut(&stack).unwrap().push(value);
    }
    pub fn pop(&mut self, stack: usize, pos_x: u32, pos_y: u32) {
        self.check(stack, 1, "pop".to_string(), pos_x, pos_y);
        let popped = self.stacks.get_mut(&stack).unwrap().pop().unwrap();
        self.push_main(popped);
    }
    pub fn push_main(&mut self, value: Value){
        self.stacks.get_mut(&0).unwrap().push(value);
    }
    pub fn pop_main(&mut self) -> Value {
        self.stacks.get_mut(&0).unwrap().pop().unwrap()
    }
    // pub fn create_stack(&mut self, stack: usize){
    //     let name = "create_stack";
    //     if self.global_stack.contains_key(&stack){
    //         panic!("Couldn't execute '{}' because of 'Error: stack {} exists'", name, stack)
    //     } else if stack < 1 {
    //         panic!("Couldn't execute '{}' because of 'Error: creating a special stack is not allowed'", name)
    //     }
    //     self.global_stack.insert(stack, vec![]);
    // }
    // pub fn duplicate_top(&mut self, stack: usize){
    //     self.check(stack, 1, "duplicate_top".to_string());
    //     let first = self.pop(stack);
    //     self.push(stack, first);
    //     self.push(stack, first);
    // }
    // pub fn duplicate_top2(&mut self, stack: usize){
    //     self.check(stack, 2, "duplicate_top2".to_string());
    //     let top = self.pop(stack);
    //     let before_top = self.pop(stack);
    //     self.push(stack, before_top);
    //     self.push(stack, top);
    //     self.push(stack, before_top);
    //     self.push(stack, top);
    // }
    // pub fn swap(&mut self, stack: usize){
    //     self.check(stack, 2, "swap".to_string());
    //     let top = self.pop(stack);
    //     let before_top = self.pop(stack);
    //     self.push(stack, top);
    //     self.push(stack, before_top);
    // }
    // pub fn rotate(&mut self, stack: usize){
    //     self.check(stack, 3, "rotate".to_string());
    //     // 3 2 1 -> 2 1 3
    //     let seq1 = self.pop(stack);
    //     let seq2 = self.pop(stack);
    //     let seq3 = self.pop(stack);
    //     self.push(stack, seq2);
    //     self.push(stack, seq1);
    //     self.push(stack, seq3);
    //
    // }
    // pub fn print_top(&mut self, stack: usize){
    //     self.duplicate_top(stack);
    //     println!("{}", self.pop(stack))
    // }
    // pub fn m_add(&mut self, stack: usize){
    //     self.check(stack, 2, "m_add".to_string());
    //     let top = self.pop(stack);
    //     let before_top = self.pop(stack);
    //     self.push(stack, before_top + top);
    // }
    // pub fn m_sub(&mut self, stack: usize){
    //     self.check(stack, 2, "m_sub".to_string());
    //     let top = self.pop(stack);
    //     let before_top = self.pop(stack);
    //     self.push(stack, before_top - top);
    // }
    // pub fn m_div(&mut self, stack: usize){
    //     self.check(stack, 2, "m_div".to_string());
    //     let top = self.pop(stack);
    //     let before_top = self.pop(stack);
    //     self.push(stack, before_top / top);
    // }
    // pub fn m_mul(&mut self, stack: usize){
    //     self.check(stack, 2, "m_mul".to_string());
    //     let top = self.pop(stack);
    //     let before_top = self.pop(stack);
    //     self.push(stack, before_top * top);
    // }
    // pub fn m_mod(&mut self, stack: usize){
    //     self.check(stack, 2, "m_mod".to_string());
    //     let top = self.pop(stack);
    //     let before_top = self.pop(stack);
    //     self.push(stack, before_top % top);
    // }
    // pub fn co_eq(&mut self, stack: usize){
    //     self.check(stack, 2, "co_eq".to_string());
    //     let top = self.pop(stack);
    //     let before_top = self.pop(stack);
    //     if before_top == top {
    //         self.push(stack, 1)
    //     } else {
    //         self.push(stack, 0)
    //     }
    // }
    // pub fn co_neq(&mut self, stack: usize){
    //     self.check(stack, 2, "co_neq".to_string());
    //     let top = self.pop(stack);
    //     let before_top = self.pop(stack);
    //     if before_top != top {
    //         self.push(stack, 1)
    //     } else {
    //         self.push(stack, 0)
    //     }
    // }
    // pub fn co_lt(&mut self, stack: usize){
    //     self.check(stack, 2, "co_lt".to_string());
    //     let top = self.pop(stack);
    //     let before_top = self.pop(stack);
    //     if before_top < top {
    //         self.push(stack, 1)
    //     } else {
    //         self.push(stack, 0)
    //     }
    // }
    // pub fn co_lt_eq(&mut self, stack: usize){
    //     self.check(stack, 2, "co_lt_eq".to_string());
    //     let top = self.pop(stack);
    //     let before_top = self.pop(stack);
    //     if before_top <= top {
    //         self.push(stack, 1)
    //     } else {
    //         self.push(stack, 0)
    //     }
    // }
    // pub fn co_gt(&mut self, stack: usize){
    //     self.check(stack, 2, "co_gt".to_string());
    //     let top = self.pop(stack);
    //     let before_top = self.pop(stack);
    //     if before_top > top {
    //         self.push(stack, 1)
    //     } else {
    //         self.push(stack, 0)
    //     }
    // }
    // pub fn co_gt_eq(&mut self, stack: usize){
    //     self.check(stack, 2, "co_gt_eq".to_string());
    //     let top = self.pop(stack);
    //     let before_top = self.pop(stack);
    //     if before_top >= top {
    //         self.push(stack, 1)
    //     } else {
    //         self.push(stack, 0)
    //     }
    // }
    // pub fn c_eq(&mut self, stack: usize) -> bool{
    //     self.check(stack, 2, "c_eq".to_string());
    //     let top = self.pop(stack);
    //     let before_top = self.pop(stack);
    //     let res = before_top == top;
    //     self.push(stack, before_top);
    //     self.push(stack, top);
    //     res
    // }
    // pub fn c_neq(&mut self, stack: usize) -> bool{
    //     self.check(stack, 2, "c_neq".to_string());
    //     let top = self.pop(stack);
    //     let before_top = self.pop(stack);
    //     let res = before_top != top;
    //     self.push(stack, before_top);
    //     self.push(stack, top);
    //     res
    // }
    // pub fn c_lt(&mut self, stack: usize) -> bool{
    //     self.check(stack, 2, "c_lt".to_string());
    //     let top = self.pop(stack);
    //     let before_top = self.pop(stack);
    //     let res = before_top < top;
    //     self.push(stack, before_top);
    //     self.push(stack, top);
    //     res
    // }
    // pub fn c_lt_eq(&mut self, stack: usize) -> bool{
    //     self.check(stack, 2, "co_lt_eq".to_string());
    //     let top = self.pop(stack);
    //     let before_top = self.pop(stack);
    //     let res = before_top <= top;
    //     self.push(stack, before_top);
    //     self.push(stack, top);
    //     res
    // }
    // pub fn c_gt(&mut self, stack: usize) -> bool{
    //     self.check(stack, 2, "c_gt".to_string());
    //     let top = self.pop(stack);
    //     let before_top = self.pop(stack);
    //     let res = before_top > top;
    //     self.push(stack, before_top);
    //     self.push(stack, top);
    //     res
    // }
    // pub fn c_gt_eq(&mut self, stack: usize) -> bool {
    //     self.check(stack, 2, "c_gt_eq".to_string());
    //     let top = self.pop(stack);
    //     let before_top = self.pop(stack);
    //     let res = before_top >= top;
    //     self.push(stack, before_top);
    //     self.push(stack, top);
    //     res
    // }
    // pub fn stack_length(&mut self, stack: usize) -> usize {
    //     self.check(stack, 0, "stack_length".to_string());
    //     self.global_stack.get(&stack).unwrap().len()
    // }
    // pub fn push_stack_length(&mut self, stack: usize) {
    //     self.check(stack, 0, "push_stack_length".to_string());
    //     self.push(stack, self.global_stack.get(&stack).unwrap().len() as i128);
    // }
    // pub fn pop_as_bool(&mut self, stack: usize) -> bool {
    //     let name = "pop_as_bool".to_string();
    //     self.check(stack, 1, name.clone());
    //     let boolean = self.pop(stack);
    //     if boolean == 1{
    //         true
    //     } else if boolean == 0 {
    //         false
    //     } else {
    //         panic!("Couldn't execute '{}' because of 'Error: expected 1 | 0 got {}'", name, boolean)
    //     }
    // }
    // pub fn move_top(&mut self, from_stack: usize, to_stack: usize){
    //     let from = self.pop(from_stack);
    //     self.push(to_stack, from)
    // }

    pub fn built_int_function_handler(&mut self, function_token: Token){
        match &*function_token.value {
            "print" => {
                let nums = 1;
                let name = "print".to_string();
                let pos_x = function_token.x;
                let pos_y = function_token.y;
                let stack: usize = 0;
                if self.stacks.get(&stack).unwrap().len() < nums as usize {
                    panic!("At line {} char {}, Couldn't execute '{}' because of 'Error: stack {} doesnt have enough items expected {} got {}'",  pos_y, pos_x, name, stack, nums, self.stacks.get(&stack).unwrap().len())
                }
                let to_print = self.pop_main();
                match to_print.val_type {
                    ValueType::Int => {println!("{}", to_print.val_int)},
                    ValueType::Float => {println!("{}", to_print.val_float)},

                }
            },
            _ => unimplemented!()
        }
    }

    pub fn new(instructions: Vec<Token>) -> Self {
        let mut new = Self {
            instructions,
            built_in_functions: vec!["print".to_string()],
            stacks: HashMap::new()
        };
        new.stacks.insert(0, vec![]);
        new
    }

    pub fn run(&mut self){
        for instruction in self.instructions.clone() {
            if instruction.is_data_type(){
                match instruction.token_type {
                    TokenType::Integer => {self.push_main(Value::new_int(instruction.value.parse::<i128>().unwrap()))},
                    TokenType::FloatingPoint => unimplemented!(),
                    TokenType::String => unimplemented!(),
                    TokenType::Boolean => unimplemented!(),
                    _ => panic!("weird error")
                }
            } else if instruction.token_type == TokenType::Identifier {
                if self.built_in_functions.contains(&instruction.value){
                    self.built_int_function_handler(instruction)
                } else {

                }
            } else if instruction.token_type == TokenType::EndOfFile {
                self.end();
            } else {
                unimplemented!()
            }
        }
    }
}