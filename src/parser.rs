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

use std::result;
use mexprp;

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Token {
    pub token_type: Type,
    pub value: String
}

#[derive(Debug, Clone, Copy, PartialEq, Hash)]
pub enum Type {
    CLS,
    TXT,
    IF,
    THEN,
    THENCOLON,
    ELSE,
    COLON,
    COMMENT,
    ARG,
    FIX,
    EQ,
    MATHOP,
    PARENTOPEN,
    PARENTCLOSE
}

pub trait HasMathOperator {
    fn has_math_operator(&mut self) -> bool;
    fn do_math(&mut self) -> result::Result<(), mexprp::errors::EvalError>;
    fn get_index(&mut self, token: &mut Token) -> usize;
}

impl HasMathOperator for std::vec::Vec<Token> {
    fn has_math_operator(&mut self) -> bool {
        for token in self {
           if token.token_type == Type::MATHOP {
               return true;
           } 
        }

        false
    }

    fn get_index(&mut self, token: &mut Token) -> usize {
        let mut return_value: usize = 0;
        for (index, token_tmp) in self.iter().enumerate() {
            if token == token_tmp {
                return_value = index;
            }
        }

        return_value
    }

    fn do_math(&mut self) -> result::Result<(), mexprp::errors::EvalError> {
        let mut has_found = false;
        let mut found_first_index = false;
        let mut math = String::new();
        let mut index: usize = 0; 
        let mut toremove: Vec<usize> = Vec::new();

        for (current_index, token) in self.iter().enumerate() {
            if token.token_type == Type::MATHOP {
                has_found = true;
            }
            
            if has_found {
                math.push_str(&token.value);
                
                if !found_first_index {
                    index = current_index;
                    found_first_index = true;
                } else{
                    toremove.push(current_index);
                }
            }
            
            
            if token.value.chars().last().unwrap() == ')' {
                break;
            }
        }
        
        for remove in toremove.iter().rev() {
            self.remove(*remove);
        }

        let math_expr =  math.to_uppercase().replace("/C(", "").replace(")", "");
        let final_value = mexprp::eval::<f64>(&math_expr)?;
        self[index] = Token {token_type: Type::ARG, value: final_value.to_string()};

        Ok(())

    }
}

impl Token {
    pub fn get_value(&mut self) -> String {
        self.value.clone()
    }

    pub fn get_type(&mut self) -> Type {
        self.token_type
    }

    pub fn is_empty(&mut self) -> bool {
        self.value.is_empty()
    }

    pub fn new(&mut self, token_type: Type, value: String) -> Self {
        Token {token_type: token_type, value: value}
    }

    pub fn is_variable(&mut self) -> bool {
        self.value.starts_with("%") && self.value.ends_with("%")
    }

    pub fn get_varname(&mut self) -> &str {
        &self.value[1..self.value.len()-1]
    }
}

pub fn tokens_contain(mut tokens: Vec<Token>, token_types: &[Type]) -> bool {
    let mut return_value: bool = true;

    for (index, token) in token_types.iter().enumerate() {
        if tokens[index].get_type() != *token {
            return_value = false;
        }
    }

    for index in token_types.len()..tokens.len() {
        if tokens[index].get_type() != token_types[token_types.len()-1] {
            return_value = false;
        }
    }

    return_value
}


pub fn tokens_retrieve(mut tokens: Vec<Token>, token_type: Type, index: isize) -> result::Result<Token, String> {
    let mut counter = 0;

    if index == -1 {
        let mut value: String = String::from("");
        let mut i: usize = 0;
        tokens.reverse();

        while tokens[i].get_type() == token_type {
            value.push_str(&tokens[i].get_value());
            i += 1;
            value.push_str(" ");
        }

        return Ok(Token{token_type: token_type, value: value[..value.len()-1].to_string()});
    }

    for mut token in tokens {
        if token.get_type() == token_type {
            if counter < index{
                counter += 1
            } else {
                return Ok(token);
            }
        }
    }

    Err("Couldn't retrieve the element !".to_string())
}

pub fn tokenize(words: std::str::SplitWhitespace) -> Vec<Token> {
    let mut v: Vec<Token> = Vec::new();
    for word in words {
        let token_type: Type = match word.to_uppercase().as_str() {
            "REM/" | "\'" | "//" => Type::COMMENT,
            "CLS/" => Type::CLS,
            "TXT/" => Type::TXT,
            "IF/" => Type::IF,
            "THEN:" => Type::THENCOLON,
            "THEN" => Type::THEN,
            "ELSE/" => Type::ELSE,
            "FIX/" => Type::FIX,
            ":" => Type::COLON,
            "=" => Type::EQ,
            "(" => Type::PARENTOPEN,
            ")" => Type::PARENTCLOSE,
            _ => {
                if word.starts_with("/C(") {
                    Type::MATHOP
                } else {
                    Type::ARG
                }
            }
        };

        let token: Token = Token {token_type: token_type, value: word.to_string()};
        v.push(token);
    }

    v
}