/*  _   _
 * | |_| | A cli translator
 * | __| | Author: NewDawn0
 * | |_| | License: MIT
 *  \__|_| https://github.com/NewDawn0/tl
 *
 * file: main.rs
 * desc: The binary release
 *
 * MIT License
 *
 * Copyright (c) 2022 NewDawn0
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the follwing conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
*/

/* Imports && Setup */
mod shared;
use shared::*;

/* Main */
fn main() {
    // Default values
    let mut from = "auto";
    let mut to = "en";

    // Parse arguments
    let args = std::env::args().skip(1).collect::<Vec<String>>(); // store args
    let mut prompt = String::new();
    
    let mut index: usize = 0;
    while index < args.len() {
        let arg = &args[index];
        match arg.as_str() {
            "-h" | "--help" => {println!("{}", HELPMSG); std::process::exit(0)},
            "-t" | "--to" => {
                if index+1 < args.len() {
                    to = args[index+1].as_str();
                    index += 1;
                } else {
                    *EXIT_CODE.lock().unwrap() = 1;
                    *ERR_MSG.lock().unwrap() = format!("Provide a language for {}", arg);
                }
            },
            "-f" | "--from" => {
                if index+1 < args.len() {
                    from = args[index+1].as_str();
                    index += 1;
                } else {
                    *EXIT_CODE.lock().unwrap() = 1;
                    *ERR_MSG.lock().unwrap() = format!("Provide a language for {}", arg);
                }
            },
            val => prompt.push_str(format!("{} ", val).as_str())
        }
        index += 1;
    }
    if prompt.is_empty() {
        *EXIT_CODE.lock().unwrap() = 1;
        *ERR_MSG.lock().unwrap() = format!("Provide a prompt");
    }
    if !ERR_MSG.lock().unwrap().is_empty() {
        eprintln!("{}{}Error{} :: {}", COLOURS.bold, COLOURS.red, COLOURS.reset, *ERR_MSG.lock().unwrap());
        std::process::exit(*EXIT_CODE.lock().unwrap())
    }
    prompt.pop(); // remove last whitespace

    // Translate
    if from != "auto" {
        match match_lang(&from) {
            Ok(lang) => from = lang,
            Err(e) => {
                eprintln!("{}{}Error{} :: {}", COLOURS.bold, COLOURS.red, COLOURS.reset, e);
                eprintln!("{}", HELP);
            }
        }
    }
    match match_lang(&to) {
        Ok(lang) => to = lang,
        Err(e) => {
            eprintln!("{}{}Error{} :: {}", COLOURS.bold, COLOURS.red, COLOURS.reset, e);
            eprintln!("{}", HELP);
        }
    }
    match parse(fetch(&prompt, &from, &to)) {
        Ok(text) => println!("{}", text),
        Err(e) => {
            eprintln!("{}{}Error{} :: {}", COLOURS.bold, COLOURS.red, COLOURS.reset, e);
            eprintln!("{}", HELP);
        }
    }

    // Parse args
    std::process::exit(*EXIT_CODE.lock().unwrap())
}
