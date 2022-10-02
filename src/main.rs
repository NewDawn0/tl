////// Imports && Setup //////
use reqwest;
use tl;
use std::env;
use std::process;

////// Static Help menues //////
const HELP: &'static str = "Try 'tl --help' for more information";
const HELPMSG: &'static str = "\x1b[1;32mNAME\x1b[0m
    \x1b[1;32mTL\x1b[0m
    \x1b[1;32mT\x1b[0mrans\x1b[1;32mL\x1b[0mate - A tool to translate sentences

\x1b[1;32mSYNOPSIS\x1b[0m
    \x1b[1;32mtl\x1b[0m [\x1b[0;34m-h\x1b[0m | \x1b[0;34m--help\x1b[0m] [<language> <string(s)> | <language> <string(s)>] 

\x1b[1;32mDESCRIPTION\x1b[0m
    A Tool that translates text using goole translate

\x1b[1;32mOPTIONS\x1b[0m
    \x1b[0;34m-h\x1b[0m        opens this screen
    \x1b[0;34m--help\x1b[0m    opens this screen

\x1b[1;32mEXAMPLE\x1b[0m
    $ \x1b[1;32mtl\x1b[0m en Mi estas ne vin. Vi estas ne min.
    > I am not you. You are not me.
";

////// Main //////
fn main() {
    // get args
    let mut args = env::args()
        .collect::<Vec<String>>();
    args.remove(0); // remove filename

    //// Parse args ////
    match args.len() {
        0 => {
            println!("tl: Insufficient amount of arguments\n{}", HELP);
            process::exit(1);
        },
        1 => match args[0].as_str() {
            "-h" | "--help" => println!("{}", HELPMSG),
            &_ => {
                println!("tl: Insufficient amount of arguments for translating\n{}", HELP);
                process::exit(1)
            }
        },
        _ =>  {
            println!("tl: Invalid ussage\n{}", HELP);
            process::exit(0)
        }
    }
    process::exit(0);
    let to: &String = &args[1].to_owned();
    let from: &str = "auto";
    let mode: &String = &args[0].to_owned();
    println!("{}", to);
    // get text
    let mut text = String::new();
    args.remove(0);
    args.remove(0);
    for arg in args.iter() {
        text.push_str(" ");
        text.push_str(&arg);
    }
    let text = rm_first_char(&text);
    match mode.as_str() {
        "-h" | "--help" => println!("{}", HELPMSG),
        &_ => translate(&to, &from, &text),
    }
    process::exit(0)
}


///// Remove first character //////
fn rm_first_char(val: &str) -> &str {
    let mut chars = val.chars();
    chars.next();
    chars.as_str()
}

////// Translate //////
fn translate(to: &str, from: &str, text: &str) {
    parse(fetch(text, from, to))
}
// Fetch page
fn fetch (text: &str, from: &str, to: &str) -> Result<String, String> {
    let url = format!("https://translate.google.com/m?tl={}&sl={}&q={}", to, from, text);
    match reqwest::blocking::get(url) {
        Ok(response) => match response.text() {
            Ok(body) => return Ok(body),
            Err(err) => return Err(err.to_string())
        },
        Err(err) => return Err(err.to_string())
    }
}
// Parse fetched
fn parse(result: Result<String, String>) {
    match result {
        Ok(body) => match tl::parse(&body.to_owned()).get_elements_by_class_name("result-container") {
            Some(element) => println!("{}", element[0].inner_text().into_owned()),
            None => {
                let error = String::from("Error whilst parsing text");
                println!("{}", error);
            }
        },
        Err(err) => println!("{}", err),
    }
}

