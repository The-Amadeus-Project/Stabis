use std::fmt::format;
use std::mem::take;
use crate::lexer::{Token, TokenType};

pub struct Compiler {
    instructions: Vec<Token>,
    defined_functions: Vec<String>,
    current_instruction: Token,
    index: i64,
    final_out: Vec<String>,
    scope_locations: Vec<(u32, u32)>,
    scopes: Vec<String>
}

impl Compiler {
    fn error(&mut self, error_body: String, pos_x: u32, pos_y: u32, call_name: String) {
        panic!("At line {} char {},\n  Couldn't execute '{}' because of\n  '{}'  ", pos_y, pos_x, call_name, error_body)
    }
    pub fn new(instructions: Vec<Token>) -> Self {
        Self {
            instructions,
            defined_functions: vec!["print".to_string(), "println".to_string(), "pop".to_string(), "push".to_string(), "create_stack".to_string(),
            "dup".to_string(), "dup2".to_string(), "swap".to_string(), "rotate".to_string(), "drop".to_string(), "main_stack_length".to_string(),
            "stack_length".to_string(), "print_stack".to_string(), "print_main_stack".to_string(), "input".to_string()],
            current_instruction: Token{
                token_type: TokenType::NullForParser,
                value: "".to_string(),
                x: 0,
                y: 0
            },
            index: -1,
            final_out: vec![],
            scopes: vec![],
            scope_locations: vec![]
        }
    }
    fn next_instruction(&mut self){
        self.index += 1;
        self.current_instruction = self.instructions[self.index as usize].clone();
    }
    pub fn run(&mut self) -> String{
        loop {
            self.next_instruction();
            if self.current_instruction.is_data_type(){
                let mut to_push = "push_main(Value::new_".to_string();
                match self.current_instruction.token_type {
                    TokenType::String => {to_push += &*format!("string({}.to_string())", self.current_instruction.true_value())},
                    TokenType::Integer => {to_push += &*format!("int({})", self.current_instruction.value)},
                    TokenType::FloatingPoint => {to_push += &*format!("float({})", self.current_instruction.value)},
                    TokenType::Boolean => {to_push += &*format!("bool({})", self.current_instruction.value)},
                    _ => unimplemented!(),
                }
                to_push += ");";
                self.final_out.push(to_push);
            } else {
                match self.current_instruction.token_type {
                    TokenType::Identifier => {
                        if self.defined_functions.contains(&self.current_instruction.value){
                            self.final_out.push(format!("{}({:?});",
                                                        self.current_instruction.value.clone(),
                                                        (self.current_instruction.x, self.current_instruction.y)));
                        } else {
                            unimplemented!()
                        }
                    },
                    TokenType::MathOperation => {
                        match &*self.current_instruction.value.clone() {
                            "+" => {
                                self.final_out.push(format!("m_add(0, {}, {});",
                                                            self.current_instruction.x, self.current_instruction.y));
                            },
                            "*" => {
                                self.final_out.push(format!("m_mul(0, {}, {});",
                                                            self.current_instruction.x, self.current_instruction.y));
                            },
                            "/" => {
                                self.final_out.push(format!("m_div(0, {}, {});",
                                                            self.current_instruction.x, self.current_instruction.y));
                            },
                            "-" => {
                                self.final_out.push(format!("m_sub(0, {}, {});",
                                                            self.current_instruction.x, self.current_instruction.y));
                            },
                            "%" => {
                                self.final_out.push(format!("m_mod(0, {}, {});",
                                                            self.current_instruction.x, self.current_instruction.y));
                            },
                            _ => { unimplemented!("{:#?}", self.current_instruction)
                            }
                        }
                    },
                    TokenType::ComparisonOperation => {
                        match &*self.current_instruction.value {
                            "==" => { self.final_out.push(format!("co_eq({}, {});",
                                                            self.current_instruction.x, self.current_instruction.y)) },
                            ">"  => { self.final_out.push(format!("co_gt({}, {});",
                                                            self.current_instruction.x, self.current_instruction.y)) },
                            "<"  => { self.final_out.push(format!("co_lt({}, {});",
                                                            self.current_instruction.x, self.current_instruction.y)) },
                            "!=" => {self.final_out.push(format!("co_neq({}, {});",
                                                            self.current_instruction.x, self.current_instruction.y)) },
                            ">=" => { self.final_out.push(format!("co_gt_eq(0{}, {});",
                                                            self.current_instruction.x, self.current_instruction.y)) },
                            "<=" => { self.final_out.push(format!("co_lt_eq({}, {});",
                                                            self.current_instruction.x, self.current_instruction.y)) },
                            _ => {unimplemented!()}
                        }
                    }
                    TokenType::Loop => {
                        self.scopes.push("loop".to_string());
                        self.final_out.push("loop".to_string());
                        self.scope_locations.push((self.current_instruction.x, self.current_instruction.y))
                    },
                    TokenType::End => {
                        if self.scopes.len() == 0 {
                            self.error("Error: Unexpected End Block".to_string(), self.current_instruction.x, self.current_instruction.y, "end".to_string());
                        }

                        let _scope = self.scopes.pop().unwrap();
                        self.final_out.push("end".to_string());
                    },
                    TokenType::If => {
                        self.scopes.push("if".to_string());
                        self.final_out.push("if".to_string());
                        self.scope_locations.push((self.current_instruction.x, self.current_instruction.y))
                    }
                    TokenType::Else => {
                        self.scopes.push("else".to_string());
                        self.final_out.push("else".to_string());
                        self.scope_locations.push((self.current_instruction.x, self.current_instruction.y))
                    }
                    TokenType::Lastly => {
                        self.scopes.push("lastly".to_string());
                        self.final_out.push("lastly".to_string());
                        self.scope_locations.push((self.current_instruction.x, self.current_instruction.y))
                    }
                    TokenType::EndOfFile => break,
                    TokenType::Break => {
                        self.final_out.push("break".to_string())
                    }

                    _ => unimplemented!()
                }
            }
        }
        self.scope_locations.reverse();
        let mut compiled = "".to_string();
        let mut un_scope_times = vec![];
        let mut scope_track = vec![];
        let mut scopes = 0;
        for part in self.final_out.clone() {
            if part == "loop" {
                compiled += &*"\t".repeat(scopes + 2);
                compiled += &*(format!("for _ in 0..self.cond_loop({:?}) ", self.scope_locations.pop().unwrap()) + "{\n");
                scopes += 1;
                scope_track.push("loop");
                un_scope_times.push(1);

            } else if part == "if" {
                compiled += &*"\t".repeat(scopes + 2);
                compiled += &*(format!("if self.cond_if({:?}) ", self.scope_locations.pop().unwrap()) + "{\n");

                if scopes > 0 {
                    let top = scope_track.pop().unwrap();
                    if top != "else" {
                        scope_track.push(top);
                        scope_track.push("if");
                        scopes += 1;
                        un_scope_times.push(1);
                    } else {
                        let d = un_scope_times.pop().unwrap() + 1;
                        un_scope_times.push(d);
                        scope_track.push("elset");
                    }
                } else {
                    scope_track.push("if");
                    scopes += 1;
                    un_scope_times.push(1);
                }
            } else if part == "lastly" {
                let loc = self.scope_locations.pop().unwrap();
                let top = scope_track.pop().unwrap();
                if top != "if" && top != "elset" {
                    self.error("Error: Unexpected Else".to_string(), loc.0, loc.1, "else".to_string())
                }
                compiled += &*"\t".repeat(scopes + 1);
                compiled += &*"} else {\n";
                scopes += 1;
                scope_track.push("lastly");
            } else if part == "else" {
                let loc = self.scope_locations.pop().unwrap();
                let top = scope_track.pop().unwrap();
                if top != "if" && top != "elset" {
                    self.error("Error: Unexpected Else".to_string(), loc.0, loc.1, "else".to_string())
                }
                compiled += &*"\t".repeat(scopes + 1);
                compiled += &*"} else {\n";
                scope_track.push("else");
            } else if part == "break" {
                compiled += &*"\t".repeat(scopes + 2);
                compiled += &*"break\n";
            } else if part == "end" {
                if scopes == 0 {
                    panic!("how?!")
                }
                scope_track.pop().unwrap();

                let de_scope_times = un_scope_times.pop().unwrap();
                for _ in 0..de_scope_times{
                    scopes -= 1;
                    compiled += &*"\t".repeat(scopes + 2);
                    compiled += "}\n";
                }
            } else {
                compiled += &*"\t".repeat(scopes + 2);
                compiled += &*format!("self.{}\n", part);
            }
        }
        std::fs::read_to_string("src/base.rs")
        .unwrap().replace(r"// CODE HERE PLEASE //", &*compiled)
    }
}