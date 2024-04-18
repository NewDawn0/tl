/*  _   _
 * | |_| | A cli translator
 * | __| | Author: NewDawn0
 * | |_| | License: MIT
 *  \__|_| https://github.com/NewDawn0/tl
 *
 * file: shared.rs
 * desc: Shared functionality between lib and bin
*/

/* Imports & Setup */
use const_format::formatcp;
use html_escape::decode_html_entities;
use reqwest::blocking::get;
use std::sync::Mutex;

/* Static globals */
pub static EXIT_CODE: Mutex<i32> = Mutex::new(0); // static exit code
pub static ERR_MSG: Mutex<String> = Mutex::new(String::new()); // static error message
pub const COLOURS: Colours = Colours {
    // const colours storing ansi colour codes
    bold: "\x1b[1m",
    reset: "\x1b[0m",
    purple: "\x1b[35m",
    blue: "\x1b[34m",
    cyan: "\x1b[36m",
    red: "\x1b[31m",
};
pub const HELP: &'static str = formatcp!(
    "Try '{}{}tl{} --help{}' for more information",
    COLOURS.bold,
    COLOURS.purple,
    COLOURS.blue,
    COLOURS.reset
);
pub const HELPMSG: &'static str = formatcp!(
    "{}{}TL{}{} - A cli translator
{}{}---------------------{}{}
Author;  NewDawn0
License: MIT
https://github.com/NewDawn0/tl

{}{}OPTIONS{}
    {}-h{}          Opens this menu
    {}--help{}      Opens this menu
    {}-t{}          sets the target language    [{}default: english{}]
    {}--to{}        sets the target language    [{}default: english{}]
    {}-f{}          sets the origin language    [{}default: auto detect{}]
    {}--from{}      sets the origin language    [{}default: auto detect{}]
    {}--file{}      Translate text from a file

{}{}EXAMPLE{}
    {}$ {}tl{} TL - Rust で NeoVim を使用して NewDawn0 によって作成されました
    {}>{} TL - Created by NewDawn0 using NeoVim in Rust

{}{}LANGUAGES{}
    {}A:{}  af - afrikaans;  sq - albanian; am - amharic; ar - arabic
        hy - armenian; as - assamese; ay - aymara
    {}B:{}  bm - bambara; eu - basque; be - belarusian; bn - bengali
        bho - bhojpuri; bs - bosnian; bg - bulgarian
    {}C:{}  ca - catalan; ceb - cebuano; ny - chichewa
        zh-CN - chinese simplified; zh-TW - chinese traditional
        co - corsican; hr - croatian; cs - czech
    {}D:{}  da - danish; dv - dhivehi; doi - dogri; nl - dutch
    {}E:{}  en - english; eo - esperanto; et - estonian; ee - ewe
    {}F:{}  fil - filipino; fi - finnish; fr - french; fy - frisian
    {}G:{}  gl - galician; ka - georgian; de - german; el - greek
        gn - guarani; gu - gujarati
    {}H:{}  ht - haitian; ha - hausa; haw - hawaiian; he - hebrew
        hi - hindi; hmn -  hmong; hun -  hungarian
    {}I:{}  isl - icelandic; ig - igbo; ilo - ilocano
        in - indonesian; ga - irish; it - italian
    {}J:{}  ja - japanese; jv -  javanese
    {}K:{}  kn - kannada; kk - kazakh; km - khmer; rw - kinyarwanda
        kok - konkani; ko - korean; kri - krio; ku - kurdish kurmanji
        ckb - kurdish sorani; ky - kyrgyz
    {}L:{}  lo - lao; lat - latin; lv - latvian; ln - lingala
        lt - lithuanian; lg - luganda; lb - luxembourgish
    {}M:{}  mk - macedonian; mai - maithili; mg - malagasy; ms - malay
        ml - malayalam; mt - maltese; mi - maori; mr - marathi
        mni - meiteilon; lus - mizo; mn - mongolian; my - myanmar
    {}N:{}  ne - nepali; no - norwegian
    {}O:{}  or - odia; om - oromo
    {}P:{}  ps - pashto; fa - persian; pol - polish
        pt - portuguese; pa - punjabi
    {}Q:{}  qwc - quechua
    {}R:{}  ro - romanian; ru - russian
    {}S:{}  sm - samoan; sa - sanskrit; gd - scots gaelic
        nso - sepedi; sr - serbian; st - sesotho; sn - shona
        sd - sindhi; si - sinhala; sk - slovak; sl - slovenian
        so - somali; es - spanish; sun - sundanese; sw - swahili
        swe - swedish
    {}T:{}  tg - tajik; ta - tamil; tat -   tatar; te - telugu; tha - thai
        ti - tigrinya; ts - tsonga; tr - turkish; tk - turkmen; tw - twi
    {}U:{}  uk - ukrainian; ur - urdu; ug - uyghur; uz - uzbek
    {}V:{}  vi - vietnamese
    {}W:{}  cy - welsh
    {}X:{}  xh - xhosa
    {}Y:{}  yi - yiddish; yo - yoruba
    {}Z:{}  zu - zulu",
    COLOURS.bold,
    COLOURS.purple,
    COLOURS.reset,
    COLOURS.purple,
    COLOURS.bold,
    COLOURS.purple,
    COLOURS.reset,
    COLOURS.purple,
    COLOURS.bold,
    COLOURS.purple,
    COLOURS.reset,
    COLOURS.cyan,
    COLOURS.reset,
    COLOURS.cyan,
    COLOURS.reset,
    COLOURS.cyan,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.cyan,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.cyan,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.cyan,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.cyan,
    COLOURS.reset,
    COLOURS.bold,
    COLOURS.purple,
    COLOURS.reset,
    COLOURS.red,
    COLOURS.purple,
    COLOURS.reset,
    COLOURS.red,
    COLOURS.reset,
    COLOURS.bold,
    COLOURS.purple,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset,
    COLOURS.blue,
    COLOURS.reset
);

/* fn match_lang: match language by its ISO code or name
 * @PARAM id: &str
 * @RVAL: Result<&str, String> */
pub fn match_lang(id: &str) -> Result<&str, String> {
    match id.to_lowercase().as_str() {
        "af" | "afrikaans" => Ok("af"),
        "sq" | "albanian" => Ok("sq"),
        "am" | "amharic" => Ok("am"),
        "ar" | "arabic" => Ok("ar"),
        "hy" | "armenian" => Ok("hy"),
        "as" | "assamese" => Ok("as"),
        "ay" | "aymara" => Ok("ay"),
        "bm" | "bambara" => Ok("bm"),
        "eu" | "basque" => Ok("eu"),
        "be" | "belarusian" => Ok("be"),
        "bn" | "bengali" => Ok("bn"),
        "bho" | "bhojpuri" => Ok("bho"),
        "bs" | "bosnian" => Ok("bs"),
        "bg" | "bulgarian" => Ok("bg"),
        "ca" | "catalan" => Ok("ca"),
        "ceb" | "cebuano" => Ok("ceb"),
        "ny" | "chichewa" => Ok("ny"),
        "zh-CN" | "chinese simplified" => Ok("zh-CN"),
        "zh-TW" | "chinese traditional" => Ok("zh-TW"),
        "co" | "corsican" => Ok("co"),
        "hr" | "croatian" => Ok("hr"),
        "cs" | "czech" => Ok("cs"),
        "da" | "danish" => Ok("da"),
        "dv" | "dhivehi" => Ok("dv"),
        "doi" | "dogri" => Ok("doi"),
        "nl" | "dutch" => Ok("nl"),
        "en" | "english" => Ok("en"),
        "eo" | "esperanto" => Ok("eo"),
        "et" | "estonian" => Ok("et"),
        "ee" | "ewe" => Ok("ee"),
        "fil" | "filipino" => Ok("fil"),
        "fi" | "finnish" => Ok("fi"),
        "fr" | "french" => Ok("fr"),
        "fy" | "frisian" => Ok("fy"),
        "gl" | "galician" => Ok("gl"),
        "ka" | "georgian" => Ok("ka"),
        "de" | "german" => Ok("de"),
        "el" | "greek" => Ok("el"),
        "gn" | "guarani" => Ok("gn"),
        "gu" | "gujarati" => Ok("gu"),
        "ht" | "haitian" => Ok("ht"),
        "ha" | "hausa" => Ok("ha"),
        "haw" | "hawaiian" => Ok("haw"),
        "he" | "hebrew" => Ok("he"),
        "hi" | "hindi" => Ok("hi"),
        "hmn" | "hmong" => Ok("hmn"),
        "hun" | "hungarian" => Ok("hun"),
        "isl" | "icelandic" => Ok("isl"),
        "ig" | "igbo" => Ok("ig"),
        "ilo" | "ilocano" => Ok("ilo"),
        "in" | "indonesian" => Ok("in"),
        "ga" | "irish" => Ok("ga"),
        "it" | "italian" => Ok("it"),
        "ja" | "japanese" => Ok("ja"),
        "jv" | "javanese" => Ok("jv"),
        "kn" | "kannada" => Ok("kn"),
        "kk" | "kazakh" => Ok("kk"),
        "km" | "khmer" => Ok("km"),
        "rw" | "kinyarwanda" => Ok("rw"),
        "kok" | "konkani" => Ok("kok"),
        "ko" | "korean" => Ok("ko"),
        "kri" | "krio" => Ok("kri"),
        "ku" | "kurdish kurmanji" => Ok("ku"),
        "ckb" | "kurdish sorani" => Ok("ckb"),
        "ky" | "kyrgyz" => Ok("ky"),
        "lo" | "lao" => Ok("lo"),
        "lat" | "latin" => Ok("lat"),
        "lv" | "latvian" => Ok("lv"),
        "ln" | "lingala" => Ok("ln"),
        "lt" | "lithuanian" => Ok("lt"),
        "lg" | "luganda" => Ok("lg"),
        "lb" | "luxembourgish" => Ok("lb"),
        "mk" | "macedonian" => Ok("mk"),
        "mai" | "maithili" => Ok("mai"),
        "mg" | "malagasy" => Ok("mg"),
        "ms" | "malay" => Ok("ms"),
        "ml" | "malayalam" => Ok("ml"),
        "mt" | "maltese" => Ok("mt"),
        "mi" | "maori" => Ok("mi"),
        "mr" | "marathi" => Ok("mr"),
        "mni" | "meiteilon" => Ok("mni"),
        "lus" | "mizo" => Ok("lus"),
        "mn" | "mongolian" => Ok("mn"),
        "my" | "myanmar" => Ok("my"),
        "ne" | "nepali" => Ok("ne"),
        "no" | "norwegian" => Ok("no"),
        "or" | "odia" => Ok("or"),
        "om" | "oromo" => Ok("om"),
        "ps" | "pashto" => Ok("ps"),
        "fa" | "persian" => Ok("fa"),
        "pol" | "polish" => Ok("pol"),
        "pt" | "portuguese" => Ok("pt"),
        "pa" | "punjabi" => Ok("pa"),
        "qwc" | "quechua" => Ok("qwc"),
        "ro" | "romanian" => Ok("ro"),
        "ru" | "russian" => Ok("ru"),
        "sm" | "samoan" => Ok("sm"),
        "sa" | "sanskrit" => Ok("sa"),
        "gd" | "scots gaelic" => Ok("gd"),
        "nso" | "sepedi" => Ok("nso"),
        "sr" | "serbian" => Ok("sr"),
        "st" | "sesotho" => Ok("st"),
        "sn" | "shona" => Ok("sn"),
        "sd" | "sindhi" => Ok("sd"),
        "si" | "sinhala" => Ok("si"),
        "sk" | "slovak" => Ok("sk"),
        "sl" | "slovenian" => Ok("sl"),
        "so" | "somali" => Ok("so"),
        "es" | "spanish" => Ok("es"),
        "sun" | "sundanese" => Ok("sun"),
        "sw" | "swahili" => Ok("sw"),
        "swe" | "swedish" => Ok("swe"),
        "tg" | "tajik" => Ok("tg"),
        "ta" | "tamil" => Ok("ta"),
        "tat" | "tatar" => Ok("tat"),
        "te" | "telugu" => Ok("te"),
        "tha" | "thai" => Ok("tha"),
        "ti" | "tigrinya" => Ok("ti"),
        "ts" | "tsonga" => Ok("ts"),
        "tr" | "turkish" => Ok("tr"),
        "tk" | "turkmen" => Ok("tk"),
        "tw" | "twi" => Ok("tw"),
        "uk" | "ukrainian" => Ok("uk"),
        "ur" | "urdu" => Ok("ur"),
        "ug" | "uyghur" => Ok("ug"),
        "uz" | "uzbek" => Ok("uz"),
        "vi" | "vietnamese" => Ok("vi"),
        "cy" | "welsh" => Ok("cy"),
        "xh" | "xhosa" => Ok("xh"),
        "yi" | "yiddish" => Ok("yi"),
        "yo" | "yoruba" => Ok("yo"),
        "zu" | "zulu" => Ok("zu"),
        lang => {
            *EXIT_CODE.lock().unwrap() = 1;
            Err(format!("Invalid language '{}'", lang))
        }
    }
}

/* fn fetch: fetches the html
 * @PARAM text: &str
 * @PARAM from: &str
 * @PARAM to: &str
 * @RVAL: Result<String, String> */
pub fn fetch(text: &str, from: &str, to: &str) -> Result<String, String> {
    let url = format!(
        "https://translate.google.com/m?tl={}&sl={}&q={}",
        to, from, text
    );
    match get(url) {
        Ok(response) => match response.text() {
            Ok(body) => return Ok(body),
            Err(err) => return Err(err.to_string()),
        },
        Err(err) => return Err(err.to_string()),
    }
}

/* fn parse: parses the fetched html
 * @PARAM result: Result<String, String>
 * @RVAL: Result<String, String> */
pub fn parse(result: Result<String, String>) -> Result<String, String> {
    match result {
        Ok(body) => {
            match tl::parse(&body.to_owned()).get_elements_by_class_name("result-container") {
                Some(element) => {
                    // decoding url encoding
                    Ok(decode_html_entities(&element[0].inner_text()).to_string())
                }
                None => {
                    *EXIT_CODE.lock().unwrap() = 1;
                    Err(String::from("Error whilst parsing text"))
                }
            }
        }
        Err(err) => {
            *EXIT_CODE.lock().unwrap() = 1;
            Err(err)
        }
    }
}

/* Struct storing term colours */
pub struct Colours {
    pub bold: &'static str,
    pub reset: &'static str,
    pub purple: &'static str,
    pub blue: &'static str,
    pub cyan: &'static str,
    pub red: &'static str,
}
