use std::collections::HashMap;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq, Eq, Debug, Clone)]
enum ValueType {
    Int,
    Float,
    String,
    Boolean,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Value {
    val_type: ValueType,
    val_int: i128,
    val_float: f64,
    val_bool: bool,
    val_string: String,
}

fn substring(str: String, start: usize, end: usize) ->  Option<String>
{
    if end <= start
    {
        return None;
    }
    let ss = (&str[start..end]).to_string();
    Option::from(ss)

}

impl Value {
    pub fn new_int(value: i128) -> Self {
        Self {
            val_type: ValueType::Int,
            val_int: value,
            val_float: 0.0,
            val_bool: false,
            val_string: "".to_string(),
        }
    }
    pub fn new_float(value: f64) -> Self {
        Self {
            val_type: ValueType::Float,
            val_int: 0,
            val_float: value,
            val_bool: false,
            val_string: "".to_string(),
        }
    }
    pub fn new_bool(value: bool) -> Self {
        Self {
            val_type: ValueType::Boolean,
            val_int: 0,
            val_float: 0.0,
            val_bool: value,
            val_string: "".to_string(),
        }
    }
    pub fn new_string(value: String) -> Self {
        Self {
            val_type: ValueType::String,
            val_int: 0,
            val_float: 0.0,
            val_bool: false,
            val_string: value,
        }
    }
    pub fn value(&self) -> String {
        match self.val_type {
            ValueType::Int => { self.val_int.to_string() }
            ValueType::Float => { self.val_float.to_string() }
            ValueType::String => { self.val_string.clone() }
            ValueType::Boolean => { self.val_bool.to_string() }
        }
    }
}

pub(crate) struct Program {
    stacks: HashMap<usize, Vec<Value>>,
}
impl Program {
    fn error(&mut self, error_body: String, pos_x: u32, pos_y: u32, call_name: String) {
        panic!(
            "At line {} char {},\n  Couldn't execute '{}' because of\n  '{}'  ",
            pos_y, pos_x, call_name, error_body
        )
    }
    fn check(
        &mut self,
        stack: usize,
        nums: u32,
        for_main: bool,
        name: String,
        pos_x: u32,
        pos_y: u32,
    ) {
        if !self.stacks.contains_key(&stack) {
            let vals = &self.stacks.values();
            self.error(
                format!(
                    "Error: stack {} doesnt exists, only stacks {:?} exists'",
                    stack, vals
                ),
                pos_x,
                pos_y,
                name,
            )
        } else if stack < 1 && !for_main {
            self.error(
                format!(
                    "Error: accessing special stack {} use different command",
                    stack
                ),
                pos_x,
                pos_y,
                name,
            )
        } else if self.stacks.get(&stack).unwrap().len() < nums as usize {
            self.error(
                format!(
                    "Error: stack {} doesnt have enough items expected {} got {}",
                    stack,
                    nums,
                    self.stacks.get(&stack).unwrap().len()
                ),
                pos_x,
                pos_y,
                name,
            )
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
    pub fn _push(&mut self, stack: usize, value: Value, pos_x: u32, pos_y: u32) {
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
    pub fn push_main(&mut self, value: Value) {
        self.stacks.get_mut(&0).unwrap().push(value);
    }
    pub fn pop_main(&mut self) -> Value {
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
    pub fn _swap(&mut self, pos_x: u32, pos_y: u32) {
        self.check(0, 2, true, "swap".to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        self.push_main(top);
        self.push_main(before_top);
    }
    pub fn _rotate(&mut self, pos_x: u32, pos_y: u32) {
        self.check(0, 3, true, "rotate".to_string(), pos_x, pos_y);
        // 3 2 1 -> 2 1 3
        let seq1 = self.pop_main();
        let seq2 = self.pop_main();
        let seq3 = self.pop_main();
        self.push_main(seq2);
        self.push_main(seq1);
        self.push_main(seq3);
    }
    pub fn m_add(&mut self, stack: usize, pos_x: u32, pos_y: u32) {
        let operation = "+";
        self.check(stack, 2, true, operation.to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        let res;
        match before_top.val_type {
            ValueType::Int => match top.val_type {
                ValueType::Int => res = Value::new_int(before_top.val_int + top.val_int),
                ValueType::Float => {
                    self.error(
                        format!(
                            "TypeError: {:?} {} {:?}",
                            before_top.val_type, operation, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::Boolean => {
                    self.error(
                        format!(
                            "TypeError: No valid math operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::String => {
                    self.error(
                        format!(
                            "TypeError: No valid {} operation for type {:?} and {:?}",
                            operation, before_top.val_type, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
            },
            ValueType::Float => match top.val_type {
                ValueType::Int => {
                    self.error(
                        format!(
                            "TypeError: {:?} {} {:?}",
                            before_top.val_type, operation, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::Float => res = Value::new_float(before_top.val_float + top.val_float),
                ValueType::Boolean => {
                    self.error(
                        format!(
                            "TypeError: No valid math operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::String => {
                    self.error(
                        format!(
                            "TypeError: No valid {} operation for type {:?} and {:?}",
                            operation, before_top.val_type, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
            },
            ValueType::Boolean => {
                self.error(
                    format!(
                        "TypeError: No valid math operation of type {:?}",
                        top.val_type
                    ),
                    pos_x,
                    pos_y,
                    operation.to_string(),
                );
                panic!()
            }
            ValueType::String => {
                self.error(
                    format!(
                        "TypeError: No valid {} operation for type {:?} and {:?}",
                        operation, before_top.val_type, top.val_type
                    ),
                    pos_x,
                    pos_y,
                    operation.to_string(),
                );
                panic!()
            }
        }

        self.push_main(res);
    }
    pub fn m_sub(&mut self, stack: usize, pos_x: u32, pos_y: u32) {
        let operation = "-";
        self.check(stack, 2, true, operation.to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        let res;
        match before_top.val_type {
            ValueType::Int => match before_top.val_type {
                ValueType::Int => res = Value::new_int(before_top.val_int - top.val_int),
                ValueType::Float => {
                    self.error(
                        format!(
                            "TypeError: {:?} {} {:?}",
                            before_top.val_type, operation, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::Boolean => {
                    self.error(
                        format!(
                            "TypeError: No valid math operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::String => {
                    self.error(
                        format!(
                            "TypeError: No valid {} operation for type {:?} and {:?}",
                            operation, before_top.val_type, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
            },
            ValueType::Float => match before_top.val_type {
                ValueType::Int => {
                    self.error(
                        format!(
                            "TypeError: {:?} {} {:?}",
                            before_top.val_type, operation, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::Float => res = Value::new_float(before_top.val_float - top.val_float),
                ValueType::Boolean => {
                    self.error(
                        format!(
                            "TypeError: No valid math operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::String => {
                    self.error(
                        format!(
                            "TypeError: No valid {} operation for type {:?} and {:?}",
                            operation, before_top.val_type, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
            },
            ValueType::Boolean => {
                self.error(
                    format!(
                        "TypeError: No valid math operation of type {:?}",
                        top.val_type
                    ),
                    pos_x,
                    pos_y,
                    operation.to_string(),
                );
                panic!()
            }
            ValueType::String => {
                self.error(
                    format!(
                        "TypeError: No valid {} operation for type {:?} and {:?}",
                        operation, before_top.val_type, top.val_type
                    ),
                    pos_x,
                    pos_y,
                    operation.to_string(),
                );
                panic!()
            }
        }

        self.push_main(res);
    }
    pub fn m_div(&mut self, stack: usize, pos_x: u32, pos_y: u32) {
        let operation = "/";
        self.check(stack, 2, true, operation.to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        let res;
        match before_top.val_type {
            ValueType::Int => match before_top.val_type {
                ValueType::Int => res = Value::new_int(before_top.val_int / top.val_int),
                ValueType::Float => {
                    self.error(
                        format!(
                            "TypeError: {:?} {} {:?}",
                            before_top.val_type, operation, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::Boolean => {
                    self.error(
                        format!(
                            "TypeError: No valid math operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::String => {
                    self.error(
                        format!(
                            "TypeError: No valid {} operation for type {:?} and {:?}",
                            operation, before_top.val_type, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
            },
            ValueType::Float => match before_top.val_type {
                ValueType::Int => {
                    self.error(
                        format!(
                            "TypeError: {:?} {} {:?}",
                            before_top.val_type, operation, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::Float => res = Value::new_float(before_top.val_float / top.val_float),
                ValueType::Boolean => {
                    self.error(
                        format!(
                            "TypeError: No valid math operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::String => {
                    self.error(
                        format!(
                            "TypeError: No valid {} operation for type {:?} and {:?}",
                            operation, before_top.val_type, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
            },
            ValueType::Boolean => {
                self.error(
                    format!(
                        "TypeError: No valid math operation of type {:?}",
                        top.val_type
                    ),
                    pos_x,
                    pos_y,
                    operation.to_string(),
                );
                panic!()
            }
            ValueType::String => {
                self.error(
                    format!(
                        "TypeError: No valid {} operation for type {:?} and {:?}",
                        operation, before_top.val_type, top.val_type
                    ),
                    pos_x,
                    pos_y,
                    operation.to_string(),
                );
                panic!()
            }
        }

        self.push_main(res);
    }
    pub fn m_mul(&mut self, stack: usize, pos_x: u32, pos_y: u32) {
        let operation = "*";
        self.check(stack, 2, true, operation.to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        let res;
        match before_top.val_type {
            ValueType::Int => match before_top.val_type {
                ValueType::Int => res = Value::new_int(before_top.val_int * top.val_int),
                ValueType::Float => {
                    self.error(
                        format!(
                            "TypeError: {:?} {} {:?}",
                            before_top.val_type, operation, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::Boolean => {
                    self.error(
                        format!(
                            "TypeError: No valid math operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::String => {
                    self.error(
                        format!(
                            "TypeError: No valid {} operation for type {:?} and {:?}",
                            operation, before_top.val_type, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
            },
            ValueType::Float => match before_top.val_type {
                ValueType::Int => {
                    self.error(
                        format!(
                            "TypeError: {:?} {} {:?}",
                            before_top.val_type, operation, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::Float => res = Value::new_float(before_top.val_float * top.val_float),
                ValueType::Boolean => {
                    self.error(
                        format!(
                            "TypeError: No valid math operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::String => {
                    self.error(
                        format!(
                            "TypeError: No valid {} operation for type {:?} and {:?}",
                            operation, before_top.val_type, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
            },
            ValueType::Boolean => {
                self.error(
                    format!(
                        "TypeError: No valid math operation of type {:?}",
                        top.val_type
                    ),
                    pos_x,
                    pos_y,
                    operation.to_string(),
                );
                panic!()
            }
            ValueType::String => {
                self.error(
                    format!(
                        "TypeError: No valid {} operation for type {:?} and {:?}",
                        operation, before_top.val_type, top.val_type
                    ),
                    pos_x,
                    pos_y,
                    operation.to_string(),
                );
                panic!()
            }
        }

        self.push_main(res);
    }
    pub fn m_mod(&mut self, stack: usize, pos_x: u32, pos_y: u32) {
        let operation = "%";
        self.check(stack, 2, true, operation.to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        // println!("{:?}\n{:?}", top, before_top);
        let res;
        match before_top.val_type {
            ValueType::Int => match before_top.val_type {
                ValueType::Int => res = Value::new_int(before_top.val_int % top.val_int),
                ValueType::Float => {
                    self.error(
                        format!(
                            "TypeError: {:?} {} {:?}",
                            before_top.val_type, operation, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::Boolean => {
                    self.error(
                        format!(
                            "TypeError: No valid math operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::String => {
                    self.error(
                        format!(
                            "TypeError: No valid {} operation for type {:?} and {:?}",
                            operation, before_top.val_type, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
            },
            ValueType::Float => match before_top.val_type {
                ValueType::Int => {
                    self.error(
                        format!(
                            "TypeError: {:?} {} {:?}",
                            before_top.val_type, operation, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::Float => res = Value::new_float(before_top.val_float % top.val_float),
                ValueType::Boolean => {
                    self.error(
                        format!(
                            "TypeError: No valid math operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::String => {
                    self.error(
                        format!(
                            "TypeError: No valid {} operation for type {:?} and {:?}",
                            operation, before_top.val_type, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
            },
            ValueType::Boolean => {
                self.error(
                    format!(
                        "TypeError: No valid math operation of type {:?}",
                        before_top.val_type
                    ),
                    pos_x,
                    pos_y,
                    operation.to_string(),
                );
                panic!()
            }
            ValueType::String => {
                self.error(
                    format!(
                        "TypeError: No valid {} operation for type {:?} and {:?}",
                        operation, before_top.val_type, top.val_type
                    ),
                    pos_x,
                    pos_y,
                    operation.to_string(),
                );
                panic!()
            }
        }

        self.push_main(res);
    }
    pub fn co_eq(&mut self, pos_x: u32, pos_y: u32) {
        self.check(0, 2, true, "co_eq".to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        self.push_main(Value::new_bool(before_top == top))
    }
    pub fn co_neq(&mut self, pos_x: u32, pos_y: u32) {
        self.check(0, 2, true, "co_neq".to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        self.push_main(Value::new_bool(before_top != top))
    }
    pub fn co_lt(&mut self, pos_x: u32, pos_y: u32) {
        let operation = "<";
        self.check(0, 2, true, operation.to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        let res;
        match before_top.val_type {
            ValueType::Int => match before_top.val_type {
                ValueType::Int => res = Value::new_bool(before_top.val_int < top.val_int),
                ValueType::Float => {
                    self.error(
                        format!(
                            "TypeError: {:?} {} {:?}",
                            before_top.val_type, operation, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::Boolean => {
                    self.error(
                        format!(
                            "TypeError: No valid comparison operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::String => {
                    self.error(
                        format!(
                            "TypeError: No valid comparison operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
            },
            ValueType::Float => match before_top.val_type {
                ValueType::Int => {
                    self.error(
                        format!(
                            "TypeError: {:?} {} {:?}",
                            before_top.val_type, operation, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::Float => res = Value::new_bool(before_top.val_float < top.val_float),
                ValueType::Boolean => {
                    self.error(
                        format!(
                            "TypeError: No valid comparison operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::String => {
                    self.error(
                        format!(
                            "TypeError: No valid comparison operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
            },
            ValueType::Boolean => {
                self.error(
                    format!(
                        "TypeError: No valid comparison operation of type {:?}",
                        top.val_type
                    ),
                    pos_x,
                    pos_y,
                    operation.to_string(),
                );
                panic!()
            }
            ValueType::String => {
                self.error(
                    format!(
                        "TypeError: No valid comparison operation of type {:?}",
                        before_top.val_type
                    ),
                    pos_x,
                    pos_y,
                    operation.to_string(),
                );
                panic!()
            }
        }
        self.push_main(res)
    }
    pub fn co_lt_eq(&mut self, pos_x: u32, pos_y: u32) {
        let operation = "<=";
        self.check(0, 2, true, operation.to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        let res;
        match before_top.val_type {
            ValueType::Int => match before_top.val_type {
                ValueType::Int => res = Value::new_bool(before_top.val_int <= top.val_int),
                ValueType::Float => {
                    self.error(
                        format!(
                            "TypeError: {:?} {} {:?}",
                            before_top.val_type, operation, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::Boolean => {
                    self.error(
                        format!(
                            "TypeError: No valid comparison operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::String => {
                    self.error(
                        format!(
                            "TypeError: No valid comparison operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
            },
            ValueType::Float => match before_top.val_type {
                ValueType::Int => {
                    self.error(
                        format!(
                            "TypeError: {:?} {} {:?}",
                            before_top.val_type, operation, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::Float => res = Value::new_bool(before_top.val_float <= top.val_float),
                ValueType::Boolean => {
                    self.error(
                        format!(
                            "TypeError: No valid comparison operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::String => {
                    self.error(
                        format!(
                            "TypeError: No valid comparison operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
            },
            ValueType::Boolean => {
                self.error(
                    format!(
                        "TypeError: No valid comparison operation of type {:?}",
                        top.val_type
                    ),
                    pos_x,
                    pos_y,
                    operation.to_string(),
                );
                panic!()
            }
            ValueType::String => {
                self.error(
                    format!(
                        "TypeError: No valid comparison operation of type {:?}",
                        before_top.val_type
                    ),
                    pos_x,
                    pos_y,
                    operation.to_string(),
                );
                panic!()
            }
        }
        self.push_main(res)
    }
    pub fn co_gt(&mut self, pos_x: u32, pos_y: u32) {
        let operation = ">";
        self.check(0, 2, true, operation.to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        let res;
        match before_top.val_type {
            ValueType::Int => match before_top.val_type {
                ValueType::Int => res = Value::new_bool(before_top.val_int > top.val_int),
                ValueType::Float => {
                    self.error(
                        format!(
                            "TypeError: {:?} {} {:?}",
                            before_top.val_type, operation, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::Boolean => {
                    self.error(
                        format!(
                            "TypeError: No valid comparison operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::String => {
                    self.error(
                        format!(
                            "TypeError: No valid comparison operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
            },
            ValueType::Float => match before_top.val_type {
                ValueType::Int => {
                    self.error(
                        format!(
                            "TypeError: {:?} {} {:?}",
                            before_top.val_type, operation, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::Float => res = Value::new_bool(before_top.val_float > top.val_float),
                ValueType::Boolean => {
                    self.error(
                        format!(
                            "TypeError: No valid comparison operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::String => {
                    self.error(
                        format!(
                            "TypeError: No valid comparison operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
            },
            ValueType::Boolean => {
                self.error(
                    format!(
                        "TypeError: No valid comparison operation of type {:?}",
                        top.val_type
                    ),
                    pos_x,
                    pos_y,
                    operation.to_string(),
                );
                panic!()
            }
            ValueType::String => {
                self.error(
                    format!(
                        "TypeError: No valid comparison operation of type {:?}",
                        before_top.val_type
                    ),
                    pos_x,
                    pos_y,
                    operation.to_string(),
                );
                panic!()
            }
        }
        self.push_main(res)
    }
    pub fn co_gt_eq(&mut self, pos_x: u32, pos_y: u32) {
        let operation = ">=";
        self.check(0, 2, true, operation.to_string(), pos_x, pos_y);
        let top = self.pop_main();
        let before_top = self.pop_main();
        let res;
        match before_top.val_type {
            ValueType::Int => match before_top.val_type {
                ValueType::Int => res = Value::new_bool(before_top.val_int >= top.val_int),
                ValueType::Float => {
                    self.error(
                        format!(
                            "TypeError: {:?} {} {:?}",
                            before_top.val_type, operation, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::Boolean => {
                    self.error(
                        format!(
                            "TypeError: No valid comparison operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::String => {
                    self.error(
                        format!(
                            "TypeError: No valid comparison operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
            },
            ValueType::Float => match before_top.val_type {
                ValueType::Int => {
                    self.error(
                        format!(
                            "TypeError: {:?} {} {:?}",
                            before_top.val_type, operation, top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::Float => res = Value::new_bool(before_top.val_float >= top.val_float),
                ValueType::Boolean => {
                    self.error(
                        format!(
                            "TypeError: No valid comparison operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
                ValueType::String => {
                    self.error(
                        format!(
                            "TypeError: No valid comparison operation of type {:?}",
                            before_top.val_type
                        ),
                        pos_x,
                        pos_y,
                        operation.to_string(),
                    );
                    panic!()
                }
            },
            ValueType::Boolean => {
                self.error(
                    format!(
                        "TypeError: No valid comparison operation of type {:?}",
                        top.val_type
                    ),
                    pos_x,
                    pos_y,
                    operation.to_string(),
                );
                panic!()
            }
            ValueType::String => {
                self.error(
                    format!(
                        "TypeError: No valid comparison operation of type {:?}",
                        before_top.val_type
                    ),
                    pos_x,
                    pos_y,
                    operation.to_string(),
                );
                panic!()
            }
        }
        self.push_main(res)
    }

    pub fn _stack_length(&mut self, stack: usize, pos_x: u32, pos_y: u32) -> usize {
        self.check(stack, 0, false, "stack_length".to_string(), pos_x, pos_y);
        self.stacks.get(&stack).unwrap().len()
    }
    fn println(&mut self, loc: (u32, u32)) {
        self.check(0, 1, true, "print".to_string(), loc.0, loc.1);
        let to_print = self.pop_main();
        match to_print.val_type {
            ValueType::Int => {
                println!("{}", to_print.val_int)
            }
            ValueType::Float => {
                println!("{}", to_print.val_float)
            }
            ValueType::Boolean => {
                println!("{}", to_print.val_bool)
            }
            ValueType::String => {
                println!("{}", to_print.val_string)
            }
        }
    }
    fn print(&mut self, loc: (u32, u32)) {
        self.check(0, 1, true, "print".to_string(), loc.0, loc.1);
        let to_print = self.pop_main();
        match to_print.val_type {
            ValueType::Int => {
                print!("{}", to_print.val_int)
            }
            ValueType::Float => {
                print!("{}", to_print.val_float)
            }
            ValueType::Boolean => {
                print!("{}", to_print.val_bool)
            }
            ValueType::String => {
                print!("{}", to_print.val_string)
            }
        }
    }
    fn input(&mut self, loc: (u32, u32)) {
        self.check(0, 1, true, "print".to_string(), loc.0, loc.1);
        let to_print = self.pop_main();
        match to_print.val_type {
            ValueType::Int => {
                print!("{}", to_print.val_int)
            }
            ValueType::Float => {
                print!("{}", to_print.val_float)
            }
            ValueType::Boolean => {
                print!("{}", to_print.val_bool)
            }
            ValueType::String => {
                print!("{}", to_print.val_string)
            }
        }
        let mut input = "".to_string();
        std::io::stdout().flush().expect("couldnt flush stdout");
        std::io::stdin()
            .read_line(&mut input)
            .expect("Couldnt read line");
        let input_len = input.len();
        if input_len > 0 {
            input = substring(input, 0, input_len - 1).unwrap()
        }
        self.push_main(Value::new_string(input));
    }
    fn push(&mut self, loc: (u32, u32)) {
        self.check(0, 2, true, "push".to_string(), loc.0, loc.1);
        let push_stack = self.pop_main();
        let push_value = self.pop_main();
        if push_stack.val_type != ValueType::Int {
            self.error(
                format!(
                    "Error: Expected push stack to be an Int got {:?}",
                    push_stack.val_type
                ),
                loc.0,
                loc.1,
                "push".to_string(),
            )
        }
        self._push(push_stack.val_int as usize, push_value, loc.0, loc.1)
    }
    fn pop(&mut self, loc: (u32, u32)) {
        self.check(0, 1, true, "pop".to_string(), loc.0, loc.1);
        let pop_stack = self.pop_main();
        if pop_stack.val_type != ValueType::Int {
            self.error(
                format!(
                    "Error: Expected pop stack to be an Int got {:?}",
                    pop_stack.val_type
                ),
                loc.0,
                loc.1,
                "pop".to_string(),
            )
        }
        self._pop(pop_stack.val_int as usize, loc.1, loc.0)
    }
    fn create_stack(&mut self, loc: (u32, u32)) {
        self.check(0, 1, true, "create_stack".to_string(), loc.0, loc.1);
        let create_stack = self.pop_main();
        if create_stack.val_type != ValueType::Int {
            self.error(
                format!(
                    "Error: Expected create stack to be an Int got {:?}",
                    create_stack.val_type
                ),
                loc.0,
                loc.1,
                "create_stack".to_string(),
            )
        }
        self._create_stack(create_stack.val_int as usize, loc.0, loc.1)
    }
    fn stack_length(&mut self, loc: (u32, u32)) {
        self.check(0, 1, true, "stack_length".to_string(), loc.0, loc.1);
        let stack_to_get_len = self.pop_main();
        if stack_to_get_len.val_type != ValueType::Int {
            self.error(
                format!(
                    "Error: Expected arg stack to be an Int got {:?}",
                    stack_to_get_len.val_type
                ),
                loc.0,
                loc.1,
                "create_stack".to_string(),
            )
        }
        let length = self._stack_length(stack_to_get_len.val_int as usize, loc.0, loc.1);
        self.push_main(Value::new_int(length as i128));
    }
    fn main_stack_length(&mut self, loc: (u32, u32)) {
        let length = self.stacks.get(&0).unwrap().len();
        self.push_main(Value::new_int(length as i128));
    }
    fn print_stack(&mut self, loc: (u32, u32)) {
        self.check(0, 1, true, "print_stack".to_string(), loc.0, loc.1);
        let stack_to_get_len = self.pop_main();
        if stack_to_get_len.val_type != ValueType::Int {
            self.error(
                format!(
                    "Error: Expected arg stack to be an Int got {:?}",
                    stack_to_get_len.val_type
                ),
                loc.0,
                loc.1,
                "create_stack".to_string(),
            )
        }
        self.check(
            stack_to_get_len.val_int as usize,
            0,
            false,
            "print_stack".to_string(),
            loc.0,
            loc.1,
        );
        let mut res = "[".to_string();
        let stack = self.stacks.get(&(stack_to_get_len.val_int as usize)).unwrap();
        for element in stack {
            res += &*(element.value() + ", ");
        }
        res = substring(res.clone(), 0, res.len() - 2).unwrap();
        res += "]";
        println!("{}", res);
    }
    fn print_main_stack(&mut self, _loc: (u32, u32)) {
        let mut res = "[".to_string();
        let stack = self.stacks.get(&0).unwrap();
        for element in stack {
            res += &*(element.value() + ", ");
        }
        res = substring(res.clone(), 0, res.len() - 2).unwrap();
        res += "]";
        println!("{}", res);
    }
    fn dup(&mut self, loc: (u32, u32)) {
        self.check(0, 1, true, "dup".to_string(), loc.0, loc.1);
        let to_duplicate = self.pop_main();
        self.push_main(to_duplicate.clone());
        self.push_main(to_duplicate);
    }
    fn dup2(&mut self, loc: (u32, u32)) {
        self.check(0, 2, true, "dup".to_string(), loc.0, loc.1);
        let top_duplicate = self.pop_main();
        let before_top_duplicate = self.pop_main();
        self.push_main(before_top_duplicate.clone());
        self.push_main(top_duplicate.clone());
        self.push_main(before_top_duplicate);
        self.push_main(top_duplicate);
    }
    fn swap(&mut self, loc: (u32, u32)) {
        self._swap(loc.0, loc.1)
    }
    fn rotate(&mut self, loc: (u32, u32)) {
        self._rotate(loc.0, loc.1)
    }
    fn drop(&mut self, loc: (u32, u32)) {
        self._drop(loc.0, loc.1)
    }
    fn cond_loop(&mut self, loc: (u32, u32)) -> i128 {
        self.check(0, 1, true, "loop".to_string(), loc.0, loc.1);
        let times = self.pop_main();
        if times.val_type != ValueType::Int {
            self.error(
                format!(
                    "TypeError: Type {:?} cannot be used for looping",
                    times.val_type
                ),
                loc.0,
                loc.1,
                "loop".to_string(),
            )
        }
        times.val_int
    }
    fn cond_if(&mut self, loc: (u32, u32)) -> bool {
        self.check(0, 1, true, "loop".to_string(), loc.0, loc.1);
        let times = self.pop_main();
        if times.val_type != ValueType::Boolean {
            self.error(
                format!(
                    "TypeError: Type {:?} cannot be used for conditions",
                    times.val_type
                ),
                loc.0,
                loc.1,
                "loop".to_string(),
            )
        }
        times.val_bool
    }
    pub fn new() -> Self {
        let mut new = Self {
            stacks: HashMap::new(),
        };
        new.stacks.insert(0, vec![]);
        new
    }
    // new to be added to docs
    fn push_time(&mut self, _loc: (u32, u32)){
        let start = SystemTime::now();
        let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards").as_millis();
        self.push_main(Value::new_int(since_the_epoch as i128));
    }
    fn as_int(&mut self, loc: (u32, u32)) {
        let operation = "as_int".to_string();
        let to_type = ValueType::Int;
        self.check(0, 1, true, "as_int".to_string(), loc.0, loc.1);
        let value = self.pop_main();
        match value.val_type {
            ValueType::Int => self.push_main(value),
            ValueType::String => {
                let as_int_res = value.val_string.parse::<i128>();
                if as_int_res.is_err() {
                    self.error(
                        format!(
                            "TypeError: Couldnt convert {:?} -> {:?}",
                            value.val_type, to_type
                        ),
                        loc.0,
                        loc.1,
                        operation,
                    )
                } else {
                    self.push_main(Value::new_int(as_int_res.unwrap()));
                }
            }
            ValueType::Float => self.push_main(Value::new_int(value.val_float as i128)),
            ValueType::Boolean => self.error(
                format!(
                    "TypeError: Cannot convert {:?} -> {:?}",
                    value.val_type, to_type
                ),
                loc.0,
                loc.1,
                operation,
            ),
        }
    }
    fn can_be_int(&mut self, loc: (u32, u32)) {
        let operation = "as_int".to_string();
        let to_type = ValueType::Int;
        self.check(0, 1, true, operation, loc.0, loc.1);
        let value = self.pop_main();
        match value.val_type {
            ValueType::Int => self.push_main(Value::new_bool(true)),
            ValueType::String => {
                let as_int_res = value.val_string.parse::<i128>();
                if as_int_res.is_err() {
                    println!("'{}'", value.val_string);
                    self.push_main(Value::new_bool(false))
                } else {
                    self.push_main(Value::new_bool(true))
                }
            }
            ValueType::Float => self.push_main(Value::new_bool(true)),
            ValueType::Boolean => self.push_main(Value::new_bool(false)),
        }
    }
    fn as_string(&mut self, loc: (u32, u32)) {
        let operation = "as_string".to_string();
        self.check(0, 1, true, operation.to_string(), loc.0, loc.1);
        let value = self.pop_main();
        match value.val_type {
            ValueType::Int => self.push_main(Value::new_string(value.val_int.to_string())),
            ValueType::String => self.push_main(value),
            ValueType::Float => self.push_main(Value::new_string(value.val_float.to_string())),
            ValueType::Boolean => self.push_main(Value::new_string(value.val_bool.to_string())),
        }
    }
    fn can_be_string(&mut self) {
        unimplemented!("just use as_string");
    }
    // fn as_bool(&mut self){
    //
    // }
    // fn can_be_bool(&mut self){
    //
    // }
    fn as_float(&mut self) {
        unimplemented!()
    }
    fn can_be_float(&mut self) {
        unimplemented!()
    }
// FUNCTION CODE HERE PLEASE //
    pub fn run(&mut self) {
// CODE HERE PLEASE //
        self.end();
    }
}

fn main() {
    Program::new().run();
}
