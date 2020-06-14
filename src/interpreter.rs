/*
 * CC+ interpreter written in Rust
 * Copyright (C) 2020 0v3rl0w & contributors
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::parser::{tokenize, Type, Token, TokenOperation, HasMathOperator};
use std::{process::Command, collections::HashMap, io::prelude::*};


pub fn interpret(script: &str, lineNbr: usize) {
    let lines = script.lines();
    let mut variables = HashMap::new();
    
    for (line_nbr, line) in lines.enumerate() {
        let words = line.split_whitespace();
        let mut buffer: String = String::new();
        let mut tokens: Vec<Token> = tokenize(words);
        let mut ifcode: Vec<String> = Vec::new();
        let mut inif: bool = false;
        let mut newline: bool = true;

        if tokens.len() == 0 {
            continue;
        }

        if tokens.has_math_operator() {
            if let Err(_) = tokens.do_math() {
                eprintln!("LINE {}: Failed to evaluate expression", line_nbr+lineNbr+1);                    
                std::process::exit(1);
            }
        }

        match tokens[0].get_type() {
            Type::TXT => {
                if let Some((_, args)) = tokens.split_first_mut() {
                    for arg in args {
                        match arg.get_type() {
                            Type::ARG => {
                                let value: &str = &arg.get_value();
                                match value {
                                    "/#R" => newline = false,

                                    "\\#PAUSE" => {
                                        print!("\n");
                                        let mut unuse = String::new();
                                        std::io::stdin().read_line(&mut unuse);
                                        break;
                                    },

                                    "\\%" => {
                                        print!("%");
                                    }

                                    _ => {
                                        print!("{}", value);
                                    }
                                }
                            }

                            Type::TEXT => {
                                if arg.is_variable() {
                                    if let Some(varvalue) = variables.get(arg.get_varname()) {
                                        print!("{} ", varvalue);
                                    } 
                                    
                                    else {
                                        eprintln!("LINE {}: Variable {} not found !",line_nbr+lineNbr+1, arg.get_varname());
                                        std::process::exit(1);
                                    }
                                } 
                                
                                else {
                                    print!("{} ", arg.get_value());
                                }
                            }

                            _ => {
                                print!("{} ", arg.get_value());
                            }
                        }
                    }
                    
                    if newline {
                        print!("\n");
                    }
                }
            },
            
            Type::IF => println!("If"),
            Type::THEN | Type::THENCOLON => println!("Then"),
            Type::ELSE => println!("Else"),

            Type::ENDIF => {
                if inif {
                    inif = false;
                    interpret(&ifcode.join("\n"), line_nbr);
                    ifcode = Vec::new();
                } else {
                    println!("LINE {}: Not in a if statement !", line_nbr+lineNbr+1);
                }
            }
            
            Type::FIX => {
                if tokens.contain(&[Type::FIX, Type::TEXT, Type::EQ, Type::TEXT]) {
                    if let Ok(mut key) = tokens.retrieve(Type::TEXT, 0) {
                        if let Ok(mut value) = tokens.retrieve(Type::TEXT, -1) {
                            variables.insert(key.get_value(), value.get_value());
                        }
                    }
                }
            }
            
            Type::CLS => {
                if cfg!(unix) {
                    let output = Command::new("clear").output().unwrap_or_else(|e| {
                        panic!("Don't know what to do! {}", e)
                    });
                    
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
            }
            
            Type::COMMENT => {}

            _ => {
                println!("Syntax error !");
            },
        };
        
    }
}