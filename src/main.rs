//! A simple CLI translation tool using Google Translate.
//!
//! This module parses command-line arguments to translate text between languages.
//! It supports direct text input, stdin piping, and listing all available languages.

mod lib; // Throws a warning but needed to compile

use clap::{ArgGroup, Parser};
use lib::{translate, Language};
use std::{
    io::{self, Read},
    process::exit,
};
use strum::IntoEnumIterator;

/// ANSI color escape for red text (used in error output).
const RED: &str = "\x1b[31;1m";

/// ANSI reset color escape.
const NC: &str = "\x1b[m";

/// Matches a string to a `Language` enum variant or prints an error and exits.
///
/// # Arguments
/// * `$lang` - A string slice containing a language code or name.
///
/// # Returns
/// A `Language` enum variant if found. Otherwise, it prints an error and exits.
macro_rules! match_lang {
    ($lang:ident) => {
        match Language::from_str($lang) {
            Some(lang) => lang,
            None => {
                eprintln!("{}error:{} `{}` is not a valid language", RED, NC, $lang);
                exit(1);
            }
        }
    };
}

/// Entry point of the CLI translator application.
///
/// Parses command-line arguments and performs actions like:
/// - Listing supported languages
/// - Reading text from stdin
/// - Translating a given text from one language to another
fn main() {
    let mut args = Cli::parse();

    // List available languages and exit
    if args.list {
        println!("Available languages:");
        for l in Language::iter().filter(|f| f.to_str().0 != "auto") {
            println!(" -> {:6} | {}", l.to_str().0, l.to_str().1);
        }
        println!(" -> auto");
        exit(0);
    }

    // Read from stdin if flag is provided
    if args.stdin {
        match read_pipe() {
            Ok(text) => args.text = Some(text),
            Err(err) => {
                eprintln!("{}error:{} {}", RED, NC, err.to_string());
                exit(1);
            }
        }
    }

    // Parse language arguments
    let arg_from = &args.from;
    let arg_to = &args.to;
    let from = match_lang!(arg_from);
    let to = match_lang!(arg_to);

    // Perform translation
    match translate(&args.text.unwrap(), from, to) {
        Ok(text) => println!("{}", text),
        Err(e) => {
            eprintln!("{}error:{} {}", RED, NC, e)
        }
    }
}

/// Command-line argument parser for the translation CLI.
///
/// Supports mutually exclusive modes: providing `--text`, `--stdin`, or `--list`.
#[derive(Parser, Debug)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args(&["text", "stdin", "list"])))]
struct Cli {
    /// Language to translate to (e.g., `en` for English).
    #[arg(short, long, default_value = "en")]
    to: String,

    /// Language to translate from (e.g., `auto` to detect automatically).
    #[arg(short, long, default_value = "auto")]
    from: String,

    /// List all supported languages.
    #[arg(short, long, default_value_t = false)]
    list: bool,

    /// Read text from standard input instead of providing it directly.
    #[arg(short, long, default_value_t = false)]
    stdin: bool,

    /// The text to translate (used if `--stdin` is not provided).
    text: Option<String>,
}

/// Reads text piped into the program via standard input.
///
/// # Returns
/// `Ok(String)` if reading succeeds, or an `io::Error` if it fails.
fn read_pipe() -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}
