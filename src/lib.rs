/*  _   _
 * | |_| | A cli translator
 * | __| | Author: NewDawn0
 * | |_| | License: MIT
 *  \__|_| https://github.com/NewDawn0/tl
 *
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

/* Crate level macros */
#![allow(unused)]

//! A translation library using google translate
//!
//! This library provides the ability to translate text from and to any given language by making
//! web-requests to translate.google.com

/* Imports */
mod shared;
use shared::*;

/// Lang enum
/// The lang enum implements all translatable languages
pub enum Lang {
    Auto,
    Af, Afrikaans,
    Sq, Albanian,
    Am, Amharic,
    Ar, Arabic,
    Hy, Armenian,
    As, Assamese,
    Ay, Aymara,
    Bm, Bambara,
    Eu, Basque,
    Be, Belarusian,
    Bn, Bengali,
    Bho, Bhojpuri,
    Bs, Bosnian,
    Bg, Bulgarian,
    Ca, Catalan,
    Ceb, Cebuano,
    Ny, Chichewa,
    ZhCN, ChineseSimplified,
    ZhTW, ChineseTraditional,
    Co, Corsican,
    Hr, Croatian,
    Cs, Czech,
    Da, Danish,
    Dv, Dhivehi,
    Doi, Dogri,
    Nl, Dutch,
    En, English,
    Eo, Esperanto,
    Et, Estonian,
    Ee, Ewe,
    Fil, Filipino,
    Fi, Finnish,
    Fr, French,
    Fy, Frisian,
    Gl, Galician,
    Ka, Georgian,
    De, German,
    El, Greek,
    Gn, Guarani,
    Gu, Gujarati,
    Ht, Haitian,
    Ha, Hausa,
    Haw, Hawaiian,
    He, Hebrew,
    Hi, Hindi,
    Hmn, Hmong,
    Hun, Hungarian,
    Isl, Icelandic,
    Ig, Igbo,
    Ilo, Ilocano,
    In, Indonesian,
    Ga, Irish,
    Jt, Italian,
    Ja, Japanese,
    Jv, Javanese,
    Kn, Kannada,
    Kk, Kazakh,
    Km, Khmer,
    Rw, Kinyarwanda,
    Kok, Konkani,
    Ko, Korean,
    Kri, Krio,
    Ku, KurdishKurmanji,
    Ckb, KurdishSorani,
    Ky, Kyrgyz,
    Lo, Lao,
    Lat, Latin,
    Lv, Latvian,
    Ln, Lingala,
    Lt, Lithuanian,
    Lg, Luganda,
    Lb, Luxembourgish,
    Mk, Lacedonian,
    Mai, Maithili,
    Mg, Malagasy,
    Ms, Malay,
    Ml, Malayalam,
    Mt, Maltese,
    Mi, Maori,
    Mr, Marathi,
    Mni, Meiteilon,
    Lus, Mizo,
    Mn, Mongolian,
    My, Myanmar,
    Ne, Nepali,
    No, Norwegian,
    Or, Odia,
    Om, Oromo,
    Ps, Pashto,
    Fa, Persian,
    Pol, Polish,
    Pt, Portuguese,
    Pa, Punjabi,
    Qwc, Quechua,
    Ro, Romanian,
    Ru, Russian,
    Sm, Samoan,
    Sa, Sanskrit,
    Gd, ScotsGaelic,
    Nso, Sepedi,
    Sr, Serbian,
    St, Sesotho,
    Sn, Shona,
    Sd, Sindhi,
    Si, Sinhala,
    Sk, Slovak,
    Sl, Slovenian,
    So, Somali,
    Es, Spanish,
    Sun, Sundanese,
    Sw, Swahili,
    Swe, Swedish,
    Tg, Tajik,
    Ta, Tamil,
    Tat, Tatar,
    Te, Telugu,
    Tha, Thai,
    Ti, Tigrinya,
    Ts, Tsonga,
    Tr, Turkish,
    Tk, Turkmen,
    Tw, Twi,
    Uk, Ukrainian,
    Ur, Urdu,
    Ug, Uyghur,
    Uz, Uzbek,
    Vi, Vietnamese,
    Cy, Welsh,
    Xh, Xhosa,
    Yi, Yiddish,
    Yo, Yoruba,
    Zu, Zulu
}

/* fn map_lang: Maps the lang enum to the language code
 * @Param language: Lang
 * @RVal: String */
fn map_lang(language: &Lang) -> &str {
    match language {
        Lang::Auto => "auto",
        Lang::Af | Lang::Afrikaans => "af",
        Lang::Sq | Lang::Albanian => "sq",
        Lang::Am | Lang::Amharic => "am",
        Lang::Ar | Lang::Arabic => "ar",
        Lang::Hy | Lang::Armenian => "hy",
        Lang::As | Lang::Assamese => "as",
        Lang::Ay | Lang::Aymara => "ay",
        Lang::Bm | Lang::Bambara => "bm",
        Lang::Eu | Lang::Basque => "eu",
        Lang::Be | Lang::Belarusian => "be",
        Lang::Bn | Lang::Bengali => "bn",
        Lang::Bho | Lang::Bhojpuri => "bho",
        Lang::Bs | Lang::Bosnian => "bs",
        Lang::Bg | Lang::Bulgarian => "bg",
        Lang::Ca | Lang::Catalan => "ca",
        Lang::Ceb | Lang::Cebuano => "ceb",
        Lang::Ny | Lang::Chichewa => "ny",
        Lang::ZhCN | Lang::ChineseSimplified => "zh-CN",
        Lang::ZhTW | Lang::ChineseTraditional => "zh-TW",
        Lang::Co | Lang::Corsican => "co",
        Lang::Hr | Lang::Croatian => "hr",
        Lang::Cs | Lang::Czech => "cs",
        Lang::Da | Lang::Danish => "da",
        Lang::Dv | Lang::Dhivehi => "dv",
        Lang::Doi | Lang::Dogri => "doi",
        Lang::Nl | Lang::Dutch => "nl",
        Lang::En | Lang::English => "en",
        Lang::Eo | Lang::Esperanto => "eo",
        Lang::Et | Lang::Estonian => "et",
        Lang::Ee | Lang::Ewe => "ee",
        Lang::Fil | Lang::Filipino => "fil",
        Lang::Fi | Lang::Finnish => "fi",
        Lang::Fr | Lang::French => "fr",
        Lang::Fy | Lang::Frisian => "fy",
        Lang::Gl | Lang::Galician => "gl",
        Lang::Ka | Lang::Georgian => "ky",
        Lang::De | Lang::German => "de",
        Lang::El | Lang::Greek => "el",
        Lang::Gn | Lang::Guarani => "gn",
        Lang::Gu | Lang::Gujarati => "gu",
        Lang::Ht | Lang::Haitian => "ht",
        Lang::Ha | Lang::Hausa => "ha",
        Lang::Haw | Lang::Hawaiian => "haw",
        Lang::He | Lang::Hebrew => "he",
        Lang::Hi | Lang::Hindi => "hi",
        Lang::Hmn | Lang::Hmong => "hmn",
        Lang::Hun | Lang::Hungarian => "hun",
        Lang::Isl | Lang::Icelandic => "iwl",
        Lang::Ig | Lang::Igbo => "it",
        Lang::Ilo | Lang::Ilocano => "ilo",
        Lang::In | Lang::Indonesian => "in",
        Lang::Ga | Lang::Irish => "ga",
        Lang::Jt | Lang::Italian => "jt",
        Lang::Ja | Lang::Japanese => "ja",
        Lang::Jv | Lang::Javanese => "jv",
        Lang::Kn | Lang::Kannada => "kn",
        Lang::Kk | Lang::Kazakh => "kk",
        Lang::Km | Lang::Khmer => ".km",
        Lang::Rw | Lang::Kinyarwanda => "rw",
        Lang::Kok | Lang::Konkani => "kok",
        Lang::Ko | Lang::Korean => "ko",
        Lang::Kri | Lang::Krio => "kri",
        Lang::Ku | Lang::KurdishKurmanji => "ku",
        Lang::Ckb | Lang::KurdishSorani => "ckb",
        Lang::Ky | Lang::Kyrgyz => "ky",
        Lang::Lo | Lang::Lao => "lo",
        Lang::Lat | Lang::Latin => "lat",
        Lang::Lv | Lang::Latvian => "lv",
        Lang::Ln | Lang::Lingala => "ln",
        Lang::Lt | Lang::Lithuanian => "lt",
        Lang::Lg | Lang::Luganda => "lg",
        Lang::Lb | Lang::Luxembourgish => "lb",
        Lang::Mk | Lang::Lacedonian => "mk",
        Lang::Mai | Lang::Maithili => "mai",
        Lang::Mg | Lang::Malagasy => "mg",
        Lang::Ms | Lang::Malay => "ms",
        Lang::Ml | Lang::Malayalam => "ml",
        Lang::Mt | Lang::Maltese => "mt",
        Lang::Mi | Lang::Maori => "mi",
        Lang::Mr | Lang::Marathi => "mr",
        Lang::Mni | Lang::Meiteilon => "mni",
        Lang::Lus | Lang::Mizo => "lus",
        Lang::Mn | Lang::Mongolian => "mn",
        Lang::My | Lang::Myanmar => "my",
        Lang::Ne | Lang::Nepali => "ne",
        Lang::No | Lang::Norwegian => "no",
        Lang::Or | Lang::Odia => "or",
        Lang::Om | Lang::Oromo => "om",
        Lang::Ps | Lang::Pashto => "ps",
        Lang::Fa | Lang::Persian => "fa",
        Lang::Pol | Lang::Polish => "pol",
        Lang::Pt | Lang::Portuguese => "pt",
        Lang::Pa | Lang::Punjabi => "pa",
        Lang::Qwc | Lang::Quechua => "qwc",
        Lang::Ro | Lang::Romanian => "ro",
        Lang::Ru | Lang::Russian => "ru",
        Lang::Sm | Lang::Samoan => "sm",
        Lang::Sa | Lang::Sanskrit => "sa",
        Lang::Gd | Lang::ScotsGaelic => "gd",
        Lang::Nso | Lang::Sepedi => "nso",
        Lang::Sr | Lang::Serbian => "sr",
        Lang::St | Lang::Sesotho => "st",
        Lang::Sn | Lang::Shona => "sn",
        Lang::Sd | Lang::Sindhi => "sd",
        Lang::Si | Lang::Sinhala => "si",
        Lang::Sk | Lang::Slovak => "sk",
        Lang::Sl | Lang::Slovenian => "sl",
        Lang::So | Lang::Somali => "so",
        Lang::Es | Lang::Spanish => "es",
        Lang::Sun | Lang::Sundanese => "sun",
        Lang::Sw | Lang::Swahili => "sw",
        Lang::Swe | Lang::Swedish => "swe",
        Lang::Tg | Lang::Tajik => "tg",
        Lang::Ta | Lang::Tamil => "ta",
        Lang::Tat | Lang::Tatar => "tat",
        Lang::Te | Lang::Telugu => "te",
        Lang::Tha | Lang::Thai => "tha",
        Lang::Ti | Lang::Tigrinya => "ti",
        Lang::Ts | Lang::Tsonga => "ts",
        Lang::Tr | Lang::Turkish => "tr",
        Lang::Tk | Lang::Turkmen => "tk",
        Lang::Tw | Lang::Twi => "tw",
        Lang::Uk | Lang::Ukrainian => "uk",
        Lang::Ur | Lang::Urdu => "ur",
        Lang::Ug | Lang::Uyghur => "ug",
        Lang::Uz | Lang::Uzbek => "uz",
        Lang::Vi | Lang::Vietnamese => "vi",
        Lang::Cy | Lang::Welsh => "cy",
        Lang::Xh | Lang::Xhosa => "xh",
        Lang::Yi | Lang::Yiddish => "yi",
        Lang::Yo | Lang::Yoruba => "yo",
        Lang::Zu | Lang::Zulu => "zu"
    }
}


/// This function translates text
///
/// # Arguments
///
/// * `prompt` - A prompt of type str which is translated
/// * `origin` - The language which text is from
/// * `target` - The language the text is to be translated to
///
/// 
/// # Example
/// use tl_lib;
///
pub fn translate(prompt: String, origin: Lang, target: Lang) -> Result<String, String> {
    let origin_lang = map_lang(&origin);
    let mut target_lang = map_lang(&target);
    match target {
        Lang::Auto => target_lang = "auto",
        _ => {}
    }
    match parse(fetch(&prompt, &origin_lang, &target_lang)) {
        Ok(res) => Ok(res),
        Err(e) => Err(e)
    }
}
