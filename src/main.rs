//// Imports && Setup ////
use reqwest;
use tl;
use std::env;
use std::process;

//// Static Help menus ////
const HELP: &'static str = "Try 'tl --help' for more information";
const HELPMSG: &'static str = "\x1b[1;32mNAME\x1b[0m
    \x1b[1;32mTL\x1b[0m
    \x1b[1;32mT\x1b[0mrans\x1b[1;32mL\x1b[0mate - A tool to translate sentences

\x1b[1;32mSYNOPSIS\x1b[0m
    \x1b[1;32mtl\x1b[0m [\x1b[0;34m-h\x1b[0m | \x1b[0;34m--help\x1b[0m] [<language or language code> <string(s)> | <language or language code> <string(s)>] 

\x1b[1;32mDESCRIPTION\x1b[0m
    A Tool that translates text using goole translate

\x1b[1;32mOPTIONS\x1b[0m
    \x1b[0;34m-h\x1b[0m             opens this screen
    \x1b[0;34m--help\x1b[0m         opens this screen
    \x1b[0;34m-f\x1b[0;33m <lang>\x1b[0m      set the origin language (defaults to auto if not provided)
    \x1b[0;34m--from\x1b[0;33m <lang>\x1b[0m  set the origin language (defaults to auto if not provided)
    \x1b[0;34m-t\x1b[0;33m <lang>\x1b[0m      set the target language (defaults to english if not provided)
    \x1b[0;34m-to\x1b[0;33m <lang>\x1b[0m     set the target language (defaults to english if not provided)

\x1b[1;32mEXAMPLE\x1b[0m
    $ \x1b[1;32mtl\x1b[0m en Mi estas ne vin. Vi estas ne min.
    > I am not you. You are not me.

\x1b[1;32mLANGUAGES\x1b[0m
";

//// Main ////
fn main() {
    //// Set default values ////
    let mut from = "auto";
    let mut to = "en";
    let text = "tl - Made by NewDawn0";

    //// get args ////
    let mut args = env::args()
        .collect::<Vec<String>>();
    let mut args_copy = env::args()
        .collect::<Vec<String>>();
    args_copy.remove(0);
    args.remove(0); // remove filename

    //// Parse args ////
    if args.len() == 0 {
        println!("tl: Insufficient amount of arguments\n{}", HELP);
        process::exit(1);
    } else {
        match args[0].as_str() {
            "-h" | "--help" => println!("{}", HELPMSG),
            &_ => {
                let mut deletion_vector: Vec<usize> = Vec::new();
                for (pos, arg) in args.iter().enumerate() {
                    match arg.as_str() {
                        "-f" | "--from" => {
                            if args.len() >= pos+1 {
                                from = match_lang(&args[pos+1]);
                                deletion_vector.push(pos);
                            } else {
                                println!("Invalid amount of arguments\n{}", HELP);
                                process::exit(1);
                            }
                        },
                        "-t" | "--to" => {
                            if args.len() >= pos+1 {
                                to = match_lang(&args[pos+1]);
                                deletion_vector.push(pos);
                            } else {
                                println!("Invalid amount of arguments\n{}", HELP);
                                process::exit(1);
                            }
                        }
                        _ => {}
                    }
                }
                for del in deletion_vector.iter() {
                    args_copy.remove(*del);
                    args_copy.remove(*del);
                }
                //// Strore text arguments as &str to translate
                let mut text = String::new();
                for arg in args_copy.iter() {
                    text.push_str(" ");
                    text.push_str(&arg);
                }
                let text = rm_first_char(&text);
                //let to: &str = match_lang("af");
                println!("from: {}\nto: {}\ntext: {}", from, to, text);
                //translate(&to, &from, &text);
            }
        }
    }
    process::exit(0);
}

//// Match language to language code or language name ////
fn match_lang(str: &str) -> &str {
    match str.to_lowercase().as_str() {
        "af" | "afrikaans" => "af",
        "sq" | "albanian" => "sq",
        "am" | "amharic" => "am",
        "ar" | "arabic" => "ar",
        "hy" | "armenian" => "hy",
        "as" | "assamese" => "as",
        "ay" | "aymara" => "ay",
        "bm" | "bambara" => "bm",
        "eu" | "basque" => "eu",
        "be" | "belarusian" => "be",
        "bn" | "bengali" => "bn",
        "bho" | "bhojpuri" => "bho",
        "bs" | "bosnian" => "bs",
        "bg" | "bulgarian" => "bg",
        "ca" | "catalan" => "ca",
        "ceb" | "cebuano" => "ceb",
        "ny" | "chichewa" => "ny",
        "zh-CN" | "chinese simplified" => "zh-CN",
        "zh-TW" | "chinese traditional" => "zh-TW",
        "co" | "corsican" => "co",
        "hr" | "croatian" => "hr",
        "cs" | "czech" => "cs",
        "da" | "danish" => "da",
        "dv" | "dhivehi" => "dv",
        "doi" | "dogri" => "doi",
        "nl" | "dutch" => "nl",
        "en" | "english" => "en",
        "eo" | "esperanto" => "eo",
        "et" | "estonian" => "et",
        "ee" | "ewe" => "ee",
        "fil" | "filipino" => "fil",
        "fi" | "finnish" => "fi",
        "fr" | "french" => "fr",
        "fy" | "frisian" => "fy",
        "gl" | "galician" => "gl",
        "ka" | "georgian" => "ka",
        "de" | "german" => "de",
        "el" | "greek" => "el",
        "gn" | "guarani" => "gn",
        "gu" | "gujarati" => "gu",
        "ht" | "haitian" => "ht",
        "ha" | "hausa" => "ha",
        "haw" | "hawaiian" => "haw",
        "he" | "hebrew" => "he",
        "hi" | "hindi" => "hi",
        "hmn" | "hmong" => "hmn",
        "hun" | "hungarian" => "hun",
        "isl" | "icelandic" => "isl",
        "ig" | "igbo" => "ig",
        "ilo" | "ilocano" => "ilo",
        "in" | "indonesian" => "in",
        "ga" | "irish" => "ga",
        "it" | "italian" => "it",
        "ja" | "japanese" => "ja",
        "jv" | "javanese" => "jv",
        "kn" | "kannada" => "kn",
        "kk" | "kazakh" => "kk",
        "km" | "khmer" => "km",
        "rw" | "kinyarwanda" => "rw",
        "kok" | "konkani" => "kok",
        "ko" | "korean" => "ko",
        "kri" | "krio" => "kri",
        "ku" | "kurdish kurmanji" => "ku",
        "ckb" | "kurdish sorani" => "ckb",
        "ky" | "kyrgyz" => "ky",
        "lo" | "lao" => "lo",
        "lat" | "latin" => "lat",
        "lv" | "latvian" => "lv",
        "ln" | "lingala" => "ln",
        "lt" | "lithuanian" => "lt",
        "lg" | "luganda" => "lg",
        "lb" | "luxembourgish" => "lb",
        "mk" | "macedonian" => "mk",
        "mai" | "maithili" => "mai",
        "mg" | "malagasy" => "mg",
        "ms" | "malay" => "ms",
        "ml" | "malayalam" => "ml",
        "mt" | "maltese" => "mt",
        "mi" | "maori" => "mi",
        "mr" | "marathi" => "mr",
        "mni" | "meiteilon" => "mni",
        "lus" | "mizo" => "lus",
        "mn" | "mongolian" => "mn",
        "my" | "myanmar" => "my",
        "ne" | "nepali" => "ne",
        "no" | "norwegian" => "no",
        "or" | "odia" => "or",
        "om" | "oromo" => "om",
        "ps" | "pashto" => "ps",
        "fa" | "persian" => "fa",
        "pol" | "polish" => "pol",
        "pt" | "portuguese" => "pt",
        "pa" | "punjabi" => "pa",
        "qwc" | "quechua" => "qwc",
        "ro" | "romanian" => "ro",
        "ru" | "russian" => "ru",
        "sm" | "samoan" => "sm",
        "sa" | "sanskrit" => "sa",
        "gd" | "scots gaelic" => "gd",
        "nso" | "sepedi" => "nso",
        "sr" | "serbian" => "sr",
        "st" | "sesotho" => "st",
        "sn" | "shona" => "sn",
        "sd" | "sindhi" => "sd",
        "si" | "sinhala" => "si",
        "sk" | "slovak" => "sk",
        "sl" | "slovenian" => "sl",
        "so" | "somali" => "so",
        "es" | "spanish" => "es",
        "sun" | "sundanese" => "sun",
        "sw" | "swahili" => "sw",
        "swe" | "swedish" => "swe",
        "tg" | "tajik" => "tg",
        "ta" | "tamil" => "ta",
        "tat" | "tatar" => "tat",
        "te" | "telugu" => "te",
        "tha" | "thai" => "tha",
        "ti" | "tigrinya" => "ti",
        "ts" | "tsonga" => "ts",
        "tr" | "turkish" => "tr",
        "tk" | "turkmen" => "tk",
        "tw" | "twi" => "tw",
        "uk" | "ukrainian" => "uk",
        "ur" | "urdu" => "ur",
        "ug" | "uyghur" => "ug",
        "uz" | "uzbek" => "uz",
        "vi" | "vietnamese" => "vi",
        "cy" | "welsh" => "cy",
        "xh" | "xhosa" => "xh",
        "yi" | "yiddish" => "yi",
        "yo" | "yoruba" => "yo",
        "zu" | "zulu" => "zu",
        __ => {
            println!("tl: Invalid language\nFind languages and language codes in the Help menu\n{}", HELP);
            process::exit(1)
        }
    }
}

//// Remove first character ////
fn rm_first_char(val: &str) -> &str {
    let mut chars = val.chars();
    chars.next();
    chars.as_str()
}

//// Translate ////
fn translate(to: &str, from: &str, text: &str) {
    parse(fetch(text, from, to))
}

//// Fetch page ////
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

//// Parse fetched ////
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
