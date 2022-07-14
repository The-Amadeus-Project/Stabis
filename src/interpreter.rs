use std::collections::HashMap;
use std::io::Write;
use crate::lexer::{Token, TokenType};

#[derive(PartialEq, Eq, Debug, Clone)]
enum ValueType {
    Int,
    Float,
    String,
    Boolean
}

#[derive(PartialEq, Debug, Clone)]
pub struct Value {
    val_type: ValueType,
    val_int: i128,
    val_float: f64,
    val_bool: bool,
    val_string: String
}


impl Value {
    pub fn new_int(value: i128) -> Self {
        Self {
            val_type: ValueType::Int,
            val_int: value,
            val_float: 0.0,
            val_bool: false,
            val_string: "".to_string()
        }
    }
    pub fn new_float(value: f64) -> Self {
        Self {
            val_type: ValueType::Float,
            val_int: 0,
            val_float: value,
            val_bool: false,
            val_string: "".to_string()
        }
    }
    pub fn new_bool(value: bool) -> Self {
        Self {
            val_type: ValueType::Boolean,
            val_int: 0,
            val_float: 0.0,
            val_bool: value,
            val_string: "".to_string()
        }
    }
    pub fn new_string(value: String) -> Self {
        Self {
            val_type: ValueType::String,
            val_int: 0,
            val_float: 0.0,
            val_bool: false,
            val_string: value
        }
    }
}

pub(crate) struct Interpreter {
    instructions: Vec<Token>,
    built_in_functions: Vec<String>,
    defined_functions: HashMap<String, u64>,
    stacks: HashMap<usize, Vec<Value>>,
    current_instruction: Token,
    index: i64,
    conditional_scopes: u32,
    calls: Vec<u64>,
    loops: Vec<(u64, u64)>,
    end_users: Vec<String>,
    last_was_lastly: bool,

}
impl Interpreter {

    fn peek(&mut self) -> TokenType {
        self.instructions.get((self.index + 1) as usize).unwrap().clone().token_type.clone()
    }
    fn error(&mut self, error_body: String, pos_x: u32, pos_y: u32, call_name: String){
        panic!("At line {} char {},\n  Couldn't execute '{}' because of\n  '{}'  ", pos_y, pos_x, call_name, error_body)
    }
    fn check(&mut self, stack: usize, nums: u32, for_main: bool, name: String, pos_x: u32, pos_y: u32){
        if !self.stacks.contains_key(&stack){
            let vals = &self.stacks.values();
            self.error(format!("Error: stack {} doesnt exists, only stacks {:?} exists'", stack, vals), pos_x, pos_y, name)
        } else if stack < 1 && !for_main {
            self.error(format!("Error: accessing special stack {} use different command", stack), pos_x, pos_y, name)
        } else if self.stacks.get(&stack).unwrap().len() < nums as usize {
            self.error(format!("Error: stack {} doesnt have enough items expected {} got {}", stack, nums, self.stacks.get(&stack).unwrap().len()), pos_x, pos_y, name)
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
        self.check(stack, 0, false, "push".to_string(), pos_x, pos_y);
        self.stacks.get_mut(&stack).unwrap().push(value);
    }
    pub fn pop(&mut self, stack: usize, pos_x: u32, pos_y: u32) {
        self.check(stack, 1, false, "pop".to_string(), pos_x, pos_y);
        let popped = self.stacks.get_mut(&stack).unwrap().pop().unwrap();
        self.push_main(popped);
    }
    pub fn drop(&mut self, pos_x: u32, pos_y: u32) {
        self.check(0, 1, true, "drop".to_string(), pos_x, pos_y);
        self.pop_main();
    }
    pub fn push_main(&mut self, value: Value){
        self.stacks.get_mut(&0).unwrap().push(value);
    }
    pub fn pop_main(&mut self) -> Value {
        self.stacks.get_mut(&0).unwrap().pop().unwrap()
    }
    pub fn create_stack(&mut self, stack: usize, pos_x: u32, pos_y: u32){
        let name = "create_stack";
        if self.stacks.contains_key(&stack){
            self.error(format!("Error: stack {} exists", stack), pos_x, pos_y, name.to_string())
        } else if stack < 1 {
            self.error(format!("Error: creating a special stack is not allowed : stack {}", stack), pos_x, pos_y, name.to_string())
        }
        self.stacks.insert(stack, vec![]);
    }
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
    pub fn swap(&mut self, pos_x: u32, pos_y: u32){
        self.check(0, 2, true, "swap".to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        self.push_main(top);
        self.push_main(before_top);
    }
    pub fn rotate(&mut self, pos_x: u32, pos_y: u32){
        self.check(0, 3, true, "rotate".to_string(), pos_x, pos_y);
        // 3 2 1 -> 2 1 3
        let seq1 = self.pop_main();
        let seq2 = self.pop_main();
        let seq3 = self.pop_main();
        self.push_main(seq2);
        self.push_main(seq1);
        self.push_main(seq3);
    }
    pub fn m_add(&mut self, stack: usize, pos_x: u32, pos_y: u32){
        let operation = "+";
        self.check(stack, 2, true, operation.to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        let res;
        match before_top.val_type {
            ValueType::Int => {
                match top.val_type {
                    ValueType::Int => {
                        res = Value::new_int(before_top.val_int + top.val_int)
                    },
                    ValueType::Float => {
                        self.error(format!("TypeError: {:?} {} {:?}", before_top.val_type, operation, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::Boolean => {
                        self.error(format!("TypeError: No valid math operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::String => {
                        self.error(format!("TypeError: No valid {} operation for type {:?} and {:?}", operation, before_top.val_type, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }
                }
            },
            ValueType::Float => {
                match top.val_type {
                    ValueType::Int => {
                        self.error(format!("TypeError: {:?} {} {:?}", before_top.val_type, operation, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::Float => {
                        res = Value::new_float(before_top.val_float + top.val_float)
                    },
                    ValueType::Boolean => {
                        self.error(format!("TypeError: No valid math operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::String => {
                        self.error(format!("TypeError: No valid {} operation for type {:?} and {:?}", operation, before_top.val_type, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }
                }
            },
            ValueType::Boolean => {
                self.error(format!("TypeError: No valid math operation of type {:?}", top.val_type), pos_x, pos_y, operation.to_string());
                panic!()
            }
            ValueType::String => {
                self.error(format!("TypeError: No valid {} operation for type {:?} and {:?}", operation, before_top.val_type, top.val_type), pos_x, pos_y, operation.to_string());
                panic!()
            }
        }

        self.push_main(res);
    }
    pub fn m_sub(&mut self, stack: usize, pos_x: u32, pos_y: u32){
        let operation = "-";
        self.check(stack, 2, true, operation.to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        let res;
        match before_top.val_type {
            ValueType::Int => {
                match before_top.val_type {
                    ValueType::Int => {
                        res = Value::new_int(before_top.val_int - top.val_int)
                    },
                    ValueType::Float => {
                        self.error(format!("TypeError: {:?} {} {:?}", before_top.val_type, operation, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::Boolean => {
                        self.error(format!("TypeError: No valid math operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::String => {
                        self.error(format!("TypeError: No valid {} operation for type {:?} and {:?}", operation, before_top.val_type, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }
                }
            },
            ValueType::Float => {
                match before_top.val_type {
                    ValueType::Int => {
                        self.error(format!("TypeError: {:?} {} {:?}", before_top.val_type, operation, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::Float => {
                        res = Value::new_float(before_top.val_float - top.val_float)
                    },
                    ValueType::Boolean => {
                        self.error(format!("TypeError: No valid math operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::String => {
                        self.error(format!("TypeError: No valid {} operation for type {:?} and {:?}", operation, before_top.val_type, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }
                }
            },
            ValueType::Boolean => {
                self.error(format!("TypeError: No valid math operation of type {:?}", top.val_type), pos_x, pos_y, operation.to_string());
                panic!()
            },
            ValueType::String => {
                self.error(format!("TypeError: No valid {} operation for type {:?} and {:?}", operation, before_top.val_type, top.val_type), pos_x, pos_y, operation.to_string());
                panic!()
            }
        }

        self.push_main(res);
    }
    pub fn m_div(&mut self, stack: usize, pos_x: u32, pos_y: u32){
        let operation = "/";
        self.check(stack, 2, true, operation.to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        let res;
        match before_top.val_type {
            ValueType::Int => {
                match before_top.val_type {
                    ValueType::Int => {
                        res = Value::new_int(before_top.val_int / top.val_int)
                    },
                    ValueType::Float => {
                        self.error(format!("TypeError: {:?} {} {:?}", before_top.val_type, operation, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::Boolean => {
                        self.error(format!("TypeError: No valid math operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::String => {
                        self.error(format!("TypeError: No valid {} operation for type {:?} and {:?}", operation, before_top.val_type, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }
                }
            },
            ValueType::Float => {
                match before_top.val_type {
                    ValueType::Int => {
                        self.error(format!("TypeError: {:?} {} {:?}", before_top.val_type, operation, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::Float => {
                        res = Value::new_float(before_top.val_float / top.val_float)
                    },
                    ValueType::Boolean => {
                        self.error(format!("TypeError: No valid math operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::String => {
                        self.error(format!("TypeError: No valid {} operation for type {:?} and {:?}", operation, before_top.val_type, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }
                }
            },
            ValueType::Boolean => {
                self.error(format!("TypeError: No valid math operation of type {:?}", top.val_type), pos_x, pos_y, operation.to_string());
                panic!()
            },
            ValueType::String => {
                self.error(format!("TypeError: No valid {} operation for type {:?} and {:?}", operation, before_top.val_type, top.val_type), pos_x, pos_y, operation.to_string());
                panic!()
            }
        }

        self.push_main(res);
    }
    pub fn m_mul(&mut self, stack: usize, pos_x: u32, pos_y: u32){
        let operation = "*";
        self.check(stack, 2, true, operation.to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        let res;
        match before_top.val_type {
            ValueType::Int => {
                match before_top.val_type {
                    ValueType::Int => {
                        res = Value::new_int(before_top.val_int * top.val_int)
                    },
                    ValueType::Float => {
                        self.error(format!("TypeError: {:?} {} {:?}", before_top.val_type, operation, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::Boolean => {
                        self.error(format!("TypeError: No valid math operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::String => {
                        self.error(format!("TypeError: No valid {} operation for type {:?} and {:?}", operation, before_top.val_type, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }
                }
            },
            ValueType::Float => {
                match before_top.val_type {
                    ValueType::Int => {
                        self.error(format!("TypeError: {:?} {} {:?}", before_top.val_type, operation, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::Float => {
                        res = Value::new_float(before_top.val_float * top.val_float)
                    },
                    ValueType::Boolean => {
                        self.error(format!("TypeError: No valid math operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::String => {
                        self.error(format!("TypeError: No valid {} operation for type {:?} and {:?}", operation, before_top.val_type, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }
                }
            },
            ValueType::Boolean => {
                self.error(format!("TypeError: No valid math operation of type {:?}", top.val_type), pos_x, pos_y, operation.to_string());
                panic!()
            },
            ValueType::String => {
                self.error(format!("TypeError: No valid {} operation for type {:?} and {:?}", operation, before_top.val_type, top.val_type), pos_x, pos_y, operation.to_string());
                panic!()
            }
        }

        self.push_main(res);
    }
    pub fn m_mod(&mut self, stack: usize, pos_x: u32, pos_y: u32){
        let operation = "%";
        self.check(stack, 2, true, operation.to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        // println!("{:?}\n{:?}", top, before_top);
        let res;
        match before_top.val_type {
            ValueType::Int => {
                match before_top.val_type {
                    ValueType::Int => {
                        res = Value::new_int(before_top.val_int % top.val_int)
                    },
                    ValueType::Float => {
                        self.error(format!("TypeError: {:?} {} {:?}", before_top.val_type, operation, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::Boolean => {
                        self.error(format!("TypeError: No valid math operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::String => {
                        self.error(format!("TypeError: No valid {} operation for type {:?} and {:?}", operation, before_top.val_type, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }
                }
            },
            ValueType::Float => {
                match before_top.val_type {
                    ValueType::Int => {
                        self.error(format!("TypeError: {:?} {} {:?}", before_top.val_type, operation, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::Float => {
                        res = Value::new_float(before_top.val_float % top.val_float)
                    },
                    ValueType::Boolean => {
                        self.error(format!("TypeError: No valid math operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::String => {
                        self.error(format!("TypeError: No valid {} operation for type {:?} and {:?}", operation, before_top.val_type, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }
                }
            },
            ValueType::Boolean => {
                self.error(format!("TypeError: No valid math operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                panic!()
            },
            ValueType::String => {
                self.error(format!("TypeError: No valid {} operation for type {:?} and {:?}", operation, before_top.val_type, top.val_type), pos_x, pos_y, operation.to_string());
                panic!()
            }
        }

        self.push_main(res);
    }
    pub fn co_eq(&mut self, pos_x: u32, pos_y: u32){
        self.check(0, 2, true,"co_eq".to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        self.push_main(Value::new_bool(before_top == top))

    }
    pub fn co_neq(&mut self, pos_x: u32, pos_y: u32){
        self.check(0, 2, true, "co_neq".to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        self.push_main(Value::new_bool(before_top != top))
    }
    pub fn co_lt(&mut self, pos_x: u32, pos_y: u32){
        let operation = "<";
        self.check(0, 2, true, operation.to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        let res;
        match before_top.val_type {
            ValueType::Int => {
                match before_top.val_type {
                    ValueType::Int => {
                        res = Value::new_bool(before_top.val_int < top.val_int)
                    },
                    ValueType::Float => {
                        self.error(format!("TypeError: {:?} {} {:?}", before_top.val_type, operation, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::Boolean => {
                        self.error(format!("TypeError: No valid comparison operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }, ValueType::String => {
                        self.error(format!("TypeError: No valid comparison operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }
                }
            },
            ValueType::Float => {
                 match before_top.val_type {
                   ValueType::Int => {
                        self.error(format!("TypeError: {:?} {} {:?}", before_top.val_type, operation, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::Float => {
                        res = Value::new_bool(before_top.val_float < top.val_float)
                    },
                    ValueType::Boolean => {
                        self.error(format!("TypeError: No valid comparison operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }, ValueType::String => {
                        self.error(format!("TypeError: No valid comparison operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }
                }
            },
            ValueType::Boolean => {
                self.error(format!("TypeError: No valid comparison operation of type {:?}", top.val_type), pos_x, pos_y, operation.to_string());
                panic!()
            }, ValueType::String => {
                self.error(format!("TypeError: No valid comparison operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                panic!()
            }
        }
        self.push_main(res)
    }
    pub fn co_lt_eq(&mut self, pos_x: u32, pos_y: u32){
        let operation = "<=";
        self.check(0, 2, true, operation.to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        let res;
        match before_top.val_type {
            ValueType::Int => {
                match before_top.val_type {
                    ValueType::Int => {
                        res = Value::new_bool(before_top.val_int <= top.val_int)
                    },
                    ValueType::Float => {
                        self.error(format!("TypeError: {:?} {} {:?}", before_top.val_type, operation, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::Boolean => {
                        self.error(format!("TypeError: No valid comparison operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }, ValueType::String => {
                        self.error(format!("TypeError: No valid comparison operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }
                }
            },
            ValueType::Float => {
                 match before_top.val_type {
                   ValueType::Int => {
                        self.error(format!("TypeError: {:?} {} {:?}", before_top.val_type, operation, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::Float => {
                        res = Value::new_bool(before_top.val_float <= top.val_float)
                    },
                    ValueType::Boolean => {
                        self.error(format!("TypeError: No valid comparison operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }, ValueType::String => {
                        self.error(format!("TypeError: No valid comparison operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }
                }
            },
            ValueType::Boolean => {
                self.error(format!("TypeError: No valid comparison operation of type {:?}", top.val_type), pos_x, pos_y, operation.to_string());
                panic!()
            }, ValueType::String => {
                self.error(format!("TypeError: No valid comparison operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                panic!()
            }
        }
        self.push_main(res)
    }
    pub fn co_gt(&mut self, pos_x: u32, pos_y: u32){
        let operation = ">";
        self.check(0, 2, true, operation.to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        let res;
        match before_top.val_type {
            ValueType::Int => {
                match before_top.val_type {
                    ValueType::Int => {
                        res = Value::new_bool(before_top.val_int > top.val_int)
                    },
                    ValueType::Float => {
                        self.error(format!("TypeError: {:?} {} {:?}", before_top.val_type, operation, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::Boolean => {
                        self.error(format!("TypeError: No valid comparison operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }, ValueType::String => {
                        self.error(format!("TypeError: No valid comparison operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }
                }
            },
            ValueType::Float => {
                 match before_top.val_type {
                   ValueType::Int => {
                        self.error(format!("TypeError: {:?} {} {:?}", before_top.val_type, operation, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::Float => {
                        res = Value::new_bool(before_top.val_float > top.val_float)
                    },
                    ValueType::Boolean => {
                        self.error(format!("TypeError: No valid comparison operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }, ValueType::String => {
                        self.error(format!("TypeError: No valid comparison operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }
                }
            },
            ValueType::Boolean => {
                self.error(format!("TypeError: No valid comparison operation of type {:?}", top.val_type), pos_x, pos_y, operation.to_string());
                panic!()
            }, ValueType::String => {
                self.error(format!("TypeError: No valid comparison operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                panic!()
            }
        }
        self.push_main(res)
    }
    pub fn co_gt_eq(&mut self, pos_x: u32, pos_y: u32){
        let operation = ">=";
        self.check(0, 2, true, operation.to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        let res;
        match before_top.val_type {
            ValueType::Int => {
                match before_top.val_type {
                    ValueType::Int => {
                        res = Value::new_bool(before_top.val_int >= top.val_int)
                    },
                    ValueType::Float => {
                        self.error(format!("TypeError: {:?} {} {:?}", before_top.val_type, operation, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::Boolean => {
                        self.error(format!("TypeError: No valid comparison operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }, ValueType::String => {
                        self.error(format!("TypeError: No valid comparison operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }
                }
            },
            ValueType::Float => {
                 match before_top.val_type {
                   ValueType::Int => {
                        self.error(format!("TypeError: {:?} {} {:?}", before_top.val_type, operation, top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    },
                    ValueType::Float => {
                        res = Value::new_bool(before_top.val_float >= top.val_float)
                    },
                    ValueType::Boolean => {
                        self.error(format!("TypeError: No valid comparison operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }, ValueType::String => {
                        self.error(format!("TypeError: No valid comparison operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                        panic!()
                    }
                }
            },
            ValueType::Boolean => {
                self.error(format!("TypeError: No valid comparison operation of type {:?}", top.val_type), pos_x, pos_y, operation.to_string());
                panic!()
            }, ValueType::String => {
                self.error(format!("TypeError: No valid comparison operation of type {:?}", before_top.val_type), pos_x, pos_y, operation.to_string());
                panic!()
            }
        }
        self.push_main(res)
    }

    pub fn stack_length(&mut self, stack: usize, pos_x: u32, pos_y: u32) -> usize {
        self.check(stack, 0, false, "stack_length".to_string(), pos_x, pos_y);
        self.stacks.get(&stack).unwrap().len()
    }

    pub fn built_int_function_handler(&mut self, function_token: Token){
        match &*function_token.value {
            "println" => {
                self.check(0, 1, true, "print".to_string(), function_token.x, function_token.y);
                let to_print = self.pop_main();
                match to_print.val_type {
                    ValueType::Int => {println!("{}", to_print.val_int)},
                    ValueType::Float => {println!("{}", to_print.val_float)},
                    ValueType::Boolean => {println!("{}", to_print.val_bool)},
                    ValueType::String => {println!("{}", to_print.val_string)}
                }
            },
            "print" => {
                self.check(0, 1, true, "print".to_string(), function_token.x, function_token.y);
                let to_print = self.pop_main();
                match to_print.val_type {
                    ValueType::Int => {print!("{}", to_print.val_int)},
                    ValueType::Float => {print!("{}", to_print.val_float)},
                    ValueType::Boolean => {print!("{}", to_print.val_bool)},
                    ValueType::String => {print!("{}", to_print.val_string)}
                }
            },
            "input" => {
                self.check(0, 1, true, "print".to_string(), function_token.x, function_token.y);
                let to_print = self.pop_main();
                match to_print.val_type {
                    ValueType::Int => {print!("{}", to_print.val_int)},
                    ValueType::Float => {print!("{}", to_print.val_float)},
                    ValueType::Boolean => {print!("{}", to_print.val_bool)},
                    ValueType::String => {print!("{}", to_print.val_string)}
                }
                let mut input = "".to_string();
                std::io::stdout().flush().expect("couldnt flush stdout");
                std::io::stdin().read_line(&mut input).expect("Couldnt read line");
                self.push_main(Value::new_string(input));

            },
            "push" => {
                self.check(0, 2, true, "push".to_string(), function_token.x, function_token.y);
                let push_stack = self.pop_main();
                let push_value = self.pop_main();
                if push_stack.val_type != ValueType::Int {
                    self.error(format!("Error: Expected push stack to be an Int got {:?}", push_stack.val_type), function_token.x, function_token.y, "push".to_string())
                }
                self.push(push_stack.val_int as usize, push_value, function_token.x, function_token.y)
            },
            "pop" => {
                self.check(0, 1, true, "pop".to_string(), function_token.x, function_token.y);
                let pop_stack = self.pop_main();
                if pop_stack.val_type != ValueType::Int {
                    self.error(format!("Error: Expected pop stack to be an Int got {:?}", pop_stack.val_type), function_token.x, function_token.y, "pop".to_string())
                }
                self.pop(pop_stack.val_int as usize, function_token.x, function_token.y)
            },
            "create_stack" => {
                self.check(0, 1, true, "create_stack".to_string(), function_token.x, function_token.y);
                let create_stack = self.pop_main();
                if create_stack.val_type != ValueType::Int {
                    self.error(format!("Error: Expected create stack to be an Int got {:?}", create_stack.val_type), function_token.x, function_token.y, "create_stack".to_string())
                }
                self.create_stack(create_stack.val_int as usize, function_token.x, function_token.y)
            },
            "stack_length" => {
                self.check(0, 1, true, "stack_length".to_string(), function_token.x, function_token.y);
                let stack_to_get_len = self.pop_main();
                if stack_to_get_len.val_type != ValueType::Int {
                    self.error(format!("Error: Expected arg stack to be an Int got {:?}", stack_to_get_len.val_type), function_token.x, function_token.y, "create_stack".to_string())
                }
                let length = self.stack_length(stack_to_get_len.val_int as usize, function_token.x, function_token.y);
                self.push_main(Value::new_int(length as i128));
            },
            "main_stack_length" => {
                let length = self.stacks.get(&0).unwrap().len();
                self.push_main(Value::new_int(length as i128));
            },
            "print_stack" => {
                self.check(0, 1, true, "print_stack".to_string(), function_token.x, function_token.y);
                let stack_to_get_len = self.pop_main();
                if stack_to_get_len.val_type != ValueType::Int {
                    self.error(format!("Error: Expected arg stack to be an Int got {:?}", stack_to_get_len.val_type), function_token.x, function_token.y, "create_stack".to_string())
                }
                self.check(stack_to_get_len.val_int as usize, 0, false, "print_stack".to_string(), function_token.x, function_token.y);
                println!("{:?}", self.stacks.get(&(stack_to_get_len.val_int as usize)));

            },
            "print_main_stack" => {
                println!("{:?}", self.stacks.get(&0));
            },
            "dup" => {
                self.check(0, 1, true, "dup".to_string(), function_token.x, function_token.y);
                let to_duplicate = self.pop_main();
                self.push_main(to_duplicate.clone());
                self.push_main(to_duplicate);
            },
            "dup2" => {
                self.check(0, 2, true, "dup".to_string(), function_token.x, function_token.y);
                let top_duplicate = self.pop_main();
                let before_top_duplicate = self.pop_main();
                self.push_main(before_top_duplicate.clone());
                self.push_main(top_duplicate.clone());
                self.push_main(before_top_duplicate);
                self.push_main(top_duplicate);
            },
            "swap" => {
                self.swap(self.current_instruction.x, self.current_instruction.y)
            },
            "rotate" => {
                self.rotate(self.current_instruction.x, self.current_instruction.y)
            },
            "drop" => {
                self.drop(self.current_instruction.x, self.current_instruction.y)
            }
            _ => unimplemented!("{}", function_token.value)
        }
    }

    fn math_handler(&mut self, math_operation: Token){
        match &*math_operation.value {
            "+" => {
                self.m_add(0, math_operation.x, math_operation.y);
            },
            "*" => {
                self.m_mul(0, math_operation.x, math_operation.y);
            },
            "/" => {
                self.m_div(0, math_operation.x, math_operation.y);
            },
            "-" => {
                self.m_sub(0, math_operation.x, math_operation.y);
            },
            "%" => {
                self.m_mod(0, math_operation.x, math_operation.y);
            },

            _ => { unimplemented!("{:#?}", math_operation)}
        }
    }
    
    fn next_instruction(&mut self){
        self.index += 1;
        self.current_instruction = self.instructions[self.index as usize].clone();
    }
    fn move_to(&mut self, index: i64){
        self.index = index;
        self.current_instruction = self.instructions[self.index as usize].clone();
    }

    pub fn new(instructions: Vec<Token>) -> Self {
        let mut new = Self {
            instructions,
            built_in_functions: vec!["print".to_string(), "println".to_string(), "pop".to_string(), "push".to_string(), "create_stack".to_string(),
            "dup".to_string(), "dup2".to_string(), "swap".to_string(), "rotate".to_string(), "drop".to_string(), "main_stack_length".to_string(),
            "stack_length".to_string(), "print_stack".to_string(), "print_main_stack".to_string(), "input".to_string()],
            stacks: HashMap::new(),
            defined_functions: HashMap::new(),
            current_instruction: Token {
                token_type: TokenType::NullForParser,
                value: "NONE".to_string(),
                x: 0,
                y: 0
            },
            index: -1,
            conditional_scopes: 0,
            calls: vec![],
            loops: vec![],
            end_users: vec![],
            last_was_lastly: false
        };
        new.stacks.insert(0, vec![]);
        new
    }

    fn _close_scope(&mut self, error: String, scope_type: &str){
        self.last_was_lastly = false;
        let mut scopes = vec![(&*scope_type, 1)];
        let mut scope_deep = 1;
        loop {
            if scope_deep == 0 {
                break
            }

            self.next_instruction();
            match self.current_instruction.token_type {
                TokenType::If => {
                    let top = scopes.pop().unwrap();
                    if top.0 == "else" {
                        scopes.push(("if", top.1))
                    } else {
                        scopes.push(top);
                        scope_deep += 1;
                        scopes.push(("if", scope_deep))
                    }
                },
                TokenType::Loop => {
                    scope_deep += 1;
                    scopes.push(("loop", scope_deep))
                },
                TokenType::End => {
                    let _top = scopes.pop().unwrap();
                    scope_deep -= 1;

                },
                TokenType::Else => {
                    let top = scopes.pop().unwrap();
                    if top.0 == "if" {
                        scopes.push(("else", top.1))
                    } else {
                        panic!("EXPECTED THE NOT")
                    }
                },
                TokenType::Lastly => {
                    let top = scopes.pop().unwrap();
                    if top.0 == "if" {
                        if top.1 == 1{
                            scope_deep -= 1;
                            self.end_users.push("lastlyTrue".to_string())
                        } else {
                            scopes.push(("lastly", top.1));
                            self.last_was_lastly = true;
                            // println!("{} {:?}", self.index, self.current_instruction);
                        }
                    } else {
                        panic!("EXPECTED THE NOT")
                    }
                },
                TokenType::EndOfFile => self.error(error.clone(), self.current_instruction.x, self.current_instruction.y, "if".to_string()),
                _ => { }
            }
        }
    }
    fn close_scope(&mut self, error: String, scope_type: &str){
        self.last_was_lastly = false;
        let mut scopes = vec![(&*scope_type, 1)];
        let mut scope_deep = 1;
        loop {
            if scope_deep == 0 {
                break
            }

            self.next_instruction();
            match self.current_instruction.token_type {
                TokenType::If => {
                    let top = scopes.pop().unwrap();
                    scopes.push(top);
                    scope_deep += 1;
                    scopes.push(("if", scope_deep))
                },
                TokenType::Loop => {
                    scope_deep += 1;
                    scopes.push(("loop", scope_deep))
                },
                TokenType::End => {
                    let _top = scopes.pop().unwrap();
                    scope_deep -= 1;
                }
                TokenType::EndOfFile => self.error(error.clone(), self.current_instruction.x, self.current_instruction.y, "if".to_string()),
                _ => { }
            }
        }
    }
    fn close_if(&mut self){
        self.last_was_lastly = false;
        let mut scopes = vec![("if", 1)];
        let mut scope_deep = 1;
        loop {
            if scope_deep == 0 {
                break
            }

            self.next_instruction();
            match self.current_instruction.token_type {
                TokenType::If => {
                    let top = scopes.pop().unwrap();
                    if top.0 == "else" {
                        scopes.push(("if", top.1))
                    } else {
                        scopes.push(top);
                        scope_deep += 1;
                        scopes.push(("if", scope_deep))
                    }
                },
                TokenType::Loop => {
                    scope_deep += 1;
                    scopes.push(("loop", scope_deep))
                },
                TokenType::End => {
                    let _top = scopes.pop().unwrap();
                    scope_deep -= 1;

                },
                TokenType::Else => {
                    let top = scopes.pop().unwrap();
                    if top.0 == "if" {
                        scopes.push(("else", top.1))
                    } else {
                        panic!("EXPECTED THE NOT")
                    }
                },
                TokenType::Lastly => {
                    let top = scopes.pop().unwrap();
                    if top.0 == "if" {
                        if top.1 == 1{
                            scope_deep -= 1;
                            self.end_users.push("lastlyTrue".to_string())
                        } else {
                            scopes.push(("lastly", top.1));
                            self.last_was_lastly = true;
                            // println!("{} {:?}", self.index, self.current_instruction);
                        }
                    } else {
                        panic!("EXPECTED THE NOT")
                    }
                },
                TokenType::EndOfFile => self.error("Error: Unexpected End for conditionals".to_string(), self.current_instruction.x, self.current_instruction.y, "if".to_string()),
                _ => { }
            }
        }
    }

    pub fn run(&mut self){
        loop {
            self.next_instruction();
            if self.current_instruction.is_data_type(){
                match self.current_instruction.token_type {
                    TokenType::Integer => {self.push_main(Value::new_int(self.current_instruction.value.parse::<i128>().unwrap()))},
                    TokenType::FloatingPoint => {self.push_main(Value::new_float(self.current_instruction.value.parse::<f64>().unwrap()))},
                    TokenType::String => {self.push_main(Value::new_string(self.current_instruction.value.clone()))},
                    TokenType::Boolean => {self.push_main(Value::new_bool(self.current_instruction.value.parse::<bool>().unwrap()))},
                    _ => panic!("weird error")
                }
            } else {
                match self.current_instruction.token_type {
                    TokenType::MathOperation => {
                        self.math_handler(self.current_instruction.clone())
                    },
                    TokenType::Identifier => {
                        if self.built_in_functions.contains(&self.current_instruction.value){
                            self.built_int_function_handler(self.current_instruction.clone())

                        } else if self.defined_functions.contains_key(&self.current_instruction.value) {
                            self.end_users.push("inst".to_string());
                            self.calls.push(self.index as u64);
                            self.move_to( self.defined_functions.get(&self.current_instruction.value).unwrap().clone() as i64)

                        } else {
                            if self.index == self.instructions.len() as i64 {
                                self.error("Error: Unexpected Identifier".to_string(), self.current_instruction.x, self.current_instruction.y, "".to_string())
                            } else if self.peek() == TokenType::Inst {
                                let inst_name = self.current_instruction.value.clone();
                                self.next_instruction();
                                let inst_index = self.index as u64;
                                self.defined_functions.insert(inst_name, inst_index);
                                unimplemented!()
                            } else {
                                unimplemented!("{:?}", self.current_instruction)
                            }
                        }
                    },
                    TokenType::If => {
                        self.check(0, 1, true, "if".to_string(), self.current_instruction.x, self.current_instruction.y);
                        let condition = self.pop_main();
                        if condition.val_type != ValueType::Boolean {
                            self.error("Error: Conditions can only accept Booleans".to_string(), self.current_instruction.x, self.current_instruction.y, "if".to_string())
                        } else if condition.val_bool {
                            self.conditional_scopes += 1;
                            self.end_users.push("if".to_string());
                        } else {
                            self.close_if()
                        }
                        // unimplemented!()
                    },
                    TokenType::Loop => {
                        // unimplemented!();
                        self.check(0, 1, true, "loop".to_string(), self.current_instruction.x, self.current_instruction.y);
                        let loop_times = self.pop_main();
                        if loop_times.val_type != ValueType::Int {
                            self.error(format!("TypeError: Type {:?} cannot be used for looping", loop_times.val_type), self.current_instruction.x, self.current_instruction.y, "loop".to_string())
                        }

                        self.end_users.push("loop".to_string());
                        self.loops.push((loop_times.val_int as u64, self.index as u64))
                    },
                    TokenType::ComparisonOperation => {
                        match &*self.current_instruction.value {
                            "==" => { self.co_eq(self.current_instruction.x, self.current_instruction.y) },
                            ">"  => { self.co_gt(self.current_instruction.x, self.current_instruction.y) },
                            "<"  => { self.co_lt(self.current_instruction.x, self.current_instruction.y) },
                            "!=" => { self.co_neq(self.current_instruction.x, self.current_instruction.y) },
                            ">=" => { self.co_gt_eq(self.current_instruction.x, self.current_instruction.y) },
                            "<=" => { self.co_lt_eq(self.current_instruction.x, self.current_instruction.y) },
                            _ => {unimplemented!()}
                        }
                    },
                    TokenType::End => {
                        if self.end_users.len() > 0 {
                            let top = self.end_users.pop().unwrap();
                            match &*top {
                                "loop" => {
                                    if self.loops.len() > 0 {
                                        let mut cond = self.loops.pop().unwrap();
                                        cond.0 -= 1;
                                        if cond.0 > 0 {
                                            self.move_to(cond.1 as i64);
                                            self.loops.push(cond);
                                            self.end_users.push(top);
                                        }
                                        // else {
                                        //     println!("{:?}", self.end_users)
                                        // }
                                    } else {
                                        panic!("LANG DEV ERROR!")
                                    }
                                    // unimplemented!()
                                },
                                "if" => {
                                    self.conditional_scopes -= 1;
                                },
                                "inst" => {
                                    let ind = self.calls.pop().unwrap() as i64;
                                    self.move_to(ind);
                                    // unimplemented!()
                                },
                                "else" => {
                                    self.conditional_scopes -= 1;
                                    // unimplemented!()
                                },
                                "lastly" => {
                                    self.conditional_scopes -= 1;
                                    // println!("{:?}", self.current_instruction)
                                    // println!(">> {:?}", self.end_users)
                                    // unimplemented!()
                                },
                                "lastlyTrue" => {},
                                _ => unimplemented!()
                            }
                        } else {
                            self.error("Error: Unexpected End Block".to_string(), self.current_instruction.x, self.current_instruction.y, "end".to_string())
                        }
                    },
                    TokenType::Else => {
                        if self.conditional_scopes < 1 {
                            self.error("Error: Unexpected Else Block".to_string(), self.current_instruction.x, self.current_instruction.y, "end".to_string())
                        }
                        let top = self.end_users.pop().unwrap();
                        match &*top {
                            "if" => {
                                self.end_users.push("else".to_string());
                            }
                            _ => panic!("UNEXPECTED!")
                        }
                        let mut num_if = 0;
                        let mut scopes = 1;
                        while scopes != 0 {
                            self.next_instruction();
                            match self.current_instruction.token_type {
                                TokenType::If => {
                                    num_if += 1;
                                    if num_if > 1 {
                                        scopes += 1;
                                    }
                                }
                                TokenType::End => {
                                    num_if -= 1;
                                    scopes -= 1
                                },
                                TokenType::EndOfFile => self.error("Error: Expected Continuation of conditionals".to_string(), self.current_instruction.x, self.current_instruction.y, "if".to_string()),
                                _ => { }
                            }
                        }

                    },
                    TokenType::Lastly => {
                        // println!("-> {} {:?}", self.index, self.current_instruction);
                        if self.conditional_scopes < 1 {
                            self.error("Error: Unexpected Else Block".to_string(), self.current_instruction.x, self.current_instruction.y, "end".to_string())
                        }
                        let top = self.end_users.pop().unwrap();
                        match &*top {
                            "if" => {
                                self.end_users.push("lastly".to_string());
                            }
                            _ => panic!("UNEXPECTED!")
                        }
                        let mut scopes = 1;
                        while scopes != 0 {
                            self.next_instruction();
                            match self.current_instruction.token_type {
                                TokenType::If => {
                                    scopes += 1;
                                }
                                TokenType::End => {
                                    scopes -= 1;
                                },
                                TokenType::EndOfFile => self.error("Error: Expected Continuation of conditionals".to_string(), self.current_instruction.x, self.current_instruction.y, "if".to_string()),
                                _ => { }
                            }
                        }
                        self.index -= 1;
                        // println!("<> {} {:?}", self.index, self.current_instruction);
                        // println!(" |- {:?}", self.end_users);
                    },
                    TokenType::Break => {
                        if self.loops.len() == 0 {
                            self.error("Error: Unexpected Break, not in a loop".to_string(), self.current_instruction.x, self.current_instruction.y, "break".to_string())
                        }
                        let mut top = self.end_users.pop().unwrap();
                        while top == "if" {
                            self.close_if();
                            self.conditional_scopes -= 1;
                            top = self.end_users.pop().unwrap();
                        }

                        let loop_now = self.loops.pop().unwrap();
                        self.move_to(loop_now.1 as i64);
                        self.close_scope("Error: Unclosed Loop".to_string(), "loop");
                    },
                    TokenType::EndOfFile => {
                        self.end();
                        break
                    },
                    _ => panic!("unknown : {:?}", self.current_instruction)
                }
            }
        }
        if self.conditional_scopes > 0 {
            self.error("Error: Unclosed Block".to_string(), self.current_instruction.x, self.current_instruction.y, "EndProgram".to_string());
        }
    }
}