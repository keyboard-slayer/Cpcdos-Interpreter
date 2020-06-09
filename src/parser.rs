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

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    token_type: Type,
    value: &'a str
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    EQ
}

impl Token<'_> {
    pub fn get_value(&mut self) -> &str {
        self.value
    }

    pub fn get_type(&mut self) -> Type {
        self.token_type
    }
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
            _ => Type::ARG
        };

        let token: Token = Token {token_type: token_type, value: word};
        v.push(token);
    }

    v
}