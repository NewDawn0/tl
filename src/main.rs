////// Imports && Setup //////
use reqwest;
use tl;
use std::env;
use std::process;

////// Static Help menus //////
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
            process::exit(1)
        }
    }
    process::exit(0);
    let to: &String = &args[1].to_owned();
    let from: &str = "auto";
    let mode: &String = &args[0].to_owned();
    println!("{}", to);

    // Collect string to translate as single string
    let mut text = String::new();
    args.remove(0);
    args.remove(0);
    for arg in args.iter() {
        text.push_str(" ");
        text.push_str(&arg);
    }
    let text = rm_first_char(&text);
}

fn match_lang(str: &str) -> &str {
    match str.to_lowercase().as_str() {
        "" | "afrikaans" => "af",
        "" | "albanian" => "sq",
        "" | "amharic" => "am",
        "" | "arabic" => "ar",
        "" | "armenian" => "hy",
        "" | "assamese" => "as",
        "" | "aymara" => "ay",
        "" | "bambara" => "bm",
        "" | "basque" => "eu",
        "" | "belarusian" => "be",
        "" | "bengali" => "bn",
        "" | "bhojpuri" => "bho",
        "" | "bosnian" => "bs",
        "" | "bulgarian" => "bg",
        "" | "catalan" => "ca",
        "" | "cebuano" => "ceb",
        "" | "chichewa" => "ny",
        "" | "chinese simplified" => "zh-CN",
        "" | "chinese traditional" => "zh-TW",
        "" | "corsican" => "co",
        "" | "croatian" => "hr",
        "" | "czech" => "cs",
        "" | "danish" => "da",
        "" | "dhivehi" => "dv",
        "" | "dogri" => "doi",
        "" | "dutch" => "nl",
        "" | "english" => "en",
        "" | "esperanto" => "eo",
        "" | "estonian" => "et",
        "" | "ewe" => "ee",
        "" | "filipino" => "fil",
        "" | "finnish" => "fi",
        "" | "french" => "fr",
        "" | "frisian" => "fy",
        "" | "galician" => "gl",
        "" | "georgian" => "ka",
        "" | "german" => "de",
        "" | "greek" => "el",
        "" | "guarani" => "gn",
        "" | "gujarati" => "gu",
        "" | "haitian" => "ht",
        "" | "hausa" => "ha",
        "" | "hawaiian" => "haw",
        "" | "hebrew" => "he",
        "" | "hindi" => "hi",
        "" | "hmong" => "hmn",
        "" | "hungarian" => "hun",
        "" | "icelandic" => "isl",
        "" | "igbo" => "ig",
        "" | "ilocano" => "ilo",
        "" | "indonesian" => "in",
        "" | "irish" => "ga",
        "" | "italian" => "it",
        "" | "japanese" => "ja",
        "" | "javanese" => "jv",
        "" | "kannada" => "kn",
        "" | "kazakh" => "kk",
        "" | "khmer" => "km",
        "" | "kinyarwanda" => "rw",
        "" | "konkani" => "kok",
        "" | "korean" => "ko",
        "" | "krio" => "kri",
        "" | "kurdish kurmanji" => "ku",
        "" | "kurdish sorani" => "ckb",
        "" | "kyrgyz" => "ky",
        "" | "lao" => "lo",
        "" | "latin" => "lat",
        "" | "latvian" => "lv",
        "" | "lingala" => "ln",
        "" | "lithuanian" => "lt",
        "" | "luganda" => "lg",
        "" | "luxembourgish" => "lb",
        "" | "macedonian" => "mk",
        "" | "maithili" => "mai",
        "" | "malagasy" => "mg",
        "" | "malay" => "ms",
        "" | "malayalam" => "ml",
        "" | "maltese" => "mt",
        "" | "maori" => "mi",
        "" | "marathi" => "mr",
        "" | "meiteilon" => "mni",
        "" | "mizo" => "lus",
        "" | "mongolian" => "mn",
        "" | "myanmar" => "my",
        "" | "nepali" => "ne",
        "" | "norwegian" => "no",
        "" | "odia" => "or",
        "" | "oromo" => "om",
        "" | "pashto" => "ps",
        "" | "persian" => "fa",
        "" | "polish" => "pol",
        "" | "portuguese" => "pt",
        "" | "punjabi" => "pa",
        "" | "quechua" => "qwc",
        "" | "romanian" => "ro",
        "" | "russian" => "ru",
        "" | "samoan" => "sm",
        "" | "sanskrit" => "sa",
        "" | "scots gaelic" => "gd",
        "" | "sepedi" => "nso",
        "" | "serbian" => "sr",
        "" | "sesotho" => "st",
        "" | "shona" => "sn",
        "" | "sindhi" => "sd",
        "" | "sinhala" => "si",
        "" | "slovak" => "sk",
        "" | "slovenian" => "sl",
        "" | "somali" => "so",
        "" | "spanish" => "es",
        "" | "sundanese" => "sun",
        "" | "swahili" => "sw",
        "" | "swedish" => "swe",
        "" | "tajik" => "tg",
        "" | "tamil" => "ta",
        "" | "tatar" => "tat",
        "" | "telugu" => "te",
        "" | "thai" => "tha",
        "" | "tigrinya" => "ti",
        "" | "tsonga" => "ts",
        "" | "turkish" => "tr",
        "" | "turkmen" => "tk",
        "" | "twi" => "tw",
        "" | "ukrainian" => "uk",
        "" | "urdu" => "ur",
        "" | "uyghur" => "ug",
        "" | "uzbek" => "uz",
        "" | "vietnamese" => "vi",
        "" | "welsh" => "cy",
        "" | "xhosa" => "xh",
        "" | "yiddish" => "yi",
        "" | "yoruba" => "yo",
        "" | "zulu" => "zu",
        _ => {
            println!("tl: Invalid language\nFind languages and language codes in the Help menu\n{}", HELP);
            process::exit(1)
        }
    }
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

