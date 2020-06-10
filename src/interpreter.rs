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

use crate::parser::{tokenize, Type, Token, tokens_contain, tokens_retrieve};
use std::{process::Command, collections::HashMap};

pub fn interpret(script: &str) {
    let lines = script.lines();
    let mut variables = HashMap::new();

    for line in lines {
        let words = line.split_whitespace();
        let mut tokens: Vec<Token> = tokenize(words);
        
        if tokens.len() == 0 {
            continue;
        }

        match tokens[0].get_type() {
            Type::ARG | Type::EQ | Type::COLON => {
                println!("Syntax error !");
            },

            Type::TXT => {
                if let Some((_, args)) = tokens.split_first_mut() {
                    for arg in args {
                        if arg.get_type() == Type::ARG {
                            if arg.is_variable() {
                                if let Some(varvalue) = variables.get(arg.get_varname()) {
                                    print!("{} ", varvalue);
                                } else {
                                    eprintln!("Variable {} not found !", arg.get_varname());
                                    std::process::exit(1)
                                }
                            } else {
                                print!("{} ", arg.get_value());
                            }
                        }
                    }
    
                    print!("\n");
                }
            },

            Type::IF => println!("If"),
            Type::THEN | Type::THENCOLON => println!("Then"),
            Type::ELSE => println!("Else"),

            Type::FIX => {
                if tokens_contain(tokens.clone(), &[Type::FIX, Type::ARG, Type::EQ, Type::ARG]) {
                    if let Ok(mut key) = tokens_retrieve(tokens.clone(), Type::ARG, 0) {
                        if let Ok(mut value) = tokens_retrieve(tokens.clone(), Type::ARG, -1) {
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
        };

    }
}