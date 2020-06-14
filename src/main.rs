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

use std::{env, process, fs, io};
use std::io::prelude::*;

mod interpreter;
pub mod parser;

fn read_script(filename: &str) -> io::Result<String> {
    let file = fs::File::open(filename)?;
    let mut buf_reader = io::BufReader::new(file);
    let mut script = String::new();
    
    buf_reader.read_to_string(&mut script)?;
    Ok(script)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("Usage: {} FILE.cpc", args[0]);
        process::exit(1);
    }

    let script = read_script(&args[1]);

    match script {
        Ok(content) => {
            interpreter::interpret(&content, 0);
            process::exit(0);
        },

        Err(content) => {
            eprintln!("{}", content);
            process::exit(1);
        },
    }
 }
