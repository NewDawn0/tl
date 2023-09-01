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
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};
use strum::AsRefStr;

/// Lang short
/// Short lang codes
#[derive(Debug, PartialEq, Eq, AsRefStr)]
pub enum LangIso {
    Af,
    Sq,
    Am,
    Ar,
    Hy,
    As,
    Ay,
    Bm,
    Eu,
    Be,
    Bn,
    Bho,
    Bs,
    Bg,
    Ca,
    Ceb,
    Ny,
    ZhCN,
    ZhTW,
    Co,
    Hr,
    Cs,
    Da,
    Dv,
    Doi,
    Nl,
    En,
    Eo,
    Et,
    Ee,
    Fil,
    Fi,
    Fr,
    Fy,
    Gl,
    Ka,
    De,
    El,
    Gn,
    Gu,
    Ht,
    Ha,
    Haw,
    He,
    Hi,
    Hmn,
    Hun,
    Isl,
    Ig,
    Ilo,
    In,
    Ga,
    Jt,
    Ja,
    Jv,
    Kn,
    Kk,
    Km,
    Rw,
    Kok,
    Ko,
    Kri,
    Ku,
    Ckb,
    Ky,
    Lo,
    Lat,
    Lv,
    Ln,
    Lt,
    Lg,
    Lb,
    Mk,
    Mai,
    Mg,
    Ms,
    Ml,
    Mt,
    Mi,
    Mr,
    Mni,
    Lus,
    Mn,
    My,
    Ne,
    No,
    Or,
    Om,
    Ps,
    Fa,
    Pol,
    Pt,
    Pa,
    Qwc,
    Ro,
    Ru,
    Sm,
    Sa,
    Gd,
    Nso,
    Sr,
    St,
    Sn,
    Sd,
    Si,
    Sk,
    Sl,
    So,
    Es,
    Sun,
    Sw,
    Swe,
    Tg,
    Ta,
    Tat,
    Te,
    Tha,
    Ti,
    Ts,
    Tr,
    Tk,
    Tw,
    Uk,
    Ur,
    Ug,
    Uz,
    Vi,
    Cy,
    Xh,
    Yi,
    Yo,
    Zu,
}

/// Lang short
/// Long lang codes
#[derive(Debug, PartialEq, Eq, AsRefStr)]
pub enum LangName {
    Afrikaans,
    Albanian,
    Amharic,
    Arabic,
    Armenian,
    Assamese,
    Aymara,
    Bambara,
    Basque,
    Belarusian,
    Bengali,
    Bhojpuri,
    Bosnian,
    Bulgarian,
    Catalan,
    Cebuano,
    Chichewa,
    ChineseSimplified,
    ChineseTraditional,
    Corsican,
    Croatian,
    Czech,
    Danish,
    Dhivehi,
    Dogri,
    Dutch,
    English,
    Esperanto,
    Estonian,
    Ewe,
    Filipino,
    Finnish,
    French,
    Frisian,
    Galician,
    Georgian,
    German,
    Greek,
    Guarani,
    Gujarati,
    Haitian,
    Hausa,
    Hawaiian,
    Hebrew,
    Hindi,
    Hmong,
    Hungarian,
    Icelandic,
    Igbo,
    Ilocano,
    Indonesian,
    Irish,
    Italian,
    Japanese,
    Javanese,
    Kannada,
    Kazakh,
    Khmer,
    Kinyarwanda,
    Konkani,
    Korean,
    Krio,
    KurdishKurmanji,
    KurdishSorani,
    Kyrgyz,
    Lao,
    Latin,
    Latvian,
    Lingala,
    Lithuanian,
    Luganda,
    Luxembourgish,
    Lacedonian,
    Maithili,
    Malagasy,
    Malay,
    Malayalam,
    Maltese,
    Maori,
    Marathi,
    Meiteilon,
    Mizo,
    Mongolian,
    Myanmar,
    Nepali,
    Norwegian,
    Odia,
    Oromo,
    Pashto,
    Persian,
    Polish,
    Portuguese,
    Punjabi,
    Quechua,
    Romanian,
    Russian,
    Samoan,
    Sanskrit,
    ScotsGaelic,
    Sepedi,
    Serbian,
    Sesotho,
    Shona,
    Sindhi,
    Sinhala,
    Slovak,
    Slovenian,
    Somali,
    Spanish,
    Sundanese,
    Swahili,
    Swedish,
    Tajik,
    Tamil,
    Tatar,
    Telugu,
    Thai,
    Tigrinya,
    Tsonga,
    Turkish,
    Turkmen,
    Twi,
    Ukrainian,
    Urdu,
    Uyghur,
    Uzbek,
    Vietnamese,
    Welsh,
    Xhosa,
    Yiddish,
    Yoruba,
    Zulu,
}

impl LangName {
    /* fn to_iso: Maps the LangName to the LangIso
     * @RVal: Result<LangIso, Box<dyn Error>> */
    pub fn to_iso(&self) -> Result<LangIso, Box<dyn Error>> {
        match self {
            LangName::Afrikaans => Ok(LangIso::Af),
            LangName::Albanian => Ok(LangIso::Sq),
            LangName::Amharic => Ok(LangIso::Am),
            LangName::Arabic => Ok(LangIso::Ar),
            LangName::Armenian => Ok(LangIso::Hy),
            LangName::Assamese => Ok(LangIso::As),
            LangName::Aymara => Ok(LangIso::Ay),
            LangName::Bambara => Ok(LangIso::Bm),
            LangName::Basque => Ok(LangIso::Eu),
            LangName::Belarusian => Ok(LangIso::Be),
            LangName::Bengali => Ok(LangIso::Bn),
            LangName::Bhojpuri => Ok(LangIso::Bho),
            LangName::Bosnian => Ok(LangIso::Bs),
            LangName::Bulgarian => Ok(LangIso::Bg),
            LangName::Catalan => Ok(LangIso::Ca),
            LangName::Cebuano => Ok(LangIso::Ceb),
            LangName::Chichewa => Ok(LangIso::Ny),
            LangName::ChineseSimplified => Ok(LangIso::ZhCN),
            LangName::ChineseTraditional => Ok(LangIso::ZhTW),
            LangName::Corsican => Ok(LangIso::Co),
            LangName::Croatian => Ok(LangIso::Hr),
            LangName::Czech => Ok(LangIso::Cs),
            LangName::Danish => Ok(LangIso::Da),
            LangName::Dhivehi => Ok(LangIso::Dv),
            LangName::Dogri => Ok(LangIso::Doi),
            LangName::Dutch => Ok(LangIso::Nl),
            LangName::English => Ok(LangIso::En),
            LangName::Esperanto => Ok(LangIso::Eo),
            LangName::Estonian => Ok(LangIso::Et),
            LangName::Ewe => Ok(LangIso::Ee),
            LangName::Filipino => Ok(LangIso::Fil),
            LangName::Finnish => Ok(LangIso::Fi),
            LangName::French => Ok(LangIso::Fr),
            LangName::Frisian => Ok(LangIso::Fy),
            LangName::Galician => Ok(LangIso::Gl),
            LangName::Georgian => Ok(LangIso::Ka),
            LangName::German => Ok(LangIso::De),
            LangName::Greek => Ok(LangIso::El),
            LangName::Guarani => Ok(LangIso::Gn),
            LangName::Gujarati => Ok(LangIso::Gu),
            LangName::Haitian => Ok(LangIso::Ht),
            LangName::Hausa => Ok(LangIso::Ha),
            LangName::Hawaiian => Ok(LangIso::Haw),
            LangName::Hebrew => Ok(LangIso::He),
            LangName::Hindi => Ok(LangIso::Hi),
            LangName::Hmong => Ok(LangIso::Hmn),
            LangName::Hungarian => Ok(LangIso::Hun),
            LangName::Icelandic => Ok(LangIso::Isl),
            LangName::Igbo => Ok(LangIso::Ig),
            LangName::Ilocano => Ok(LangIso::Ilo),
            LangName::Indonesian => Ok(LangIso::In),
            LangName::Irish => Ok(LangIso::Ga),
            LangName::Italian => Ok(LangIso::Jt),
            LangName::Japanese => Ok(LangIso::Ja),
            LangName::Javanese => Ok(LangIso::Jv),
            LangName::Kannada => Ok(LangIso::Kn),
            LangName::Kazakh => Ok(LangIso::Kk),
            LangName::Khmer => Ok(LangIso::Km),
            LangName::Kinyarwanda => Ok(LangIso::Rw),
            LangName::Konkani => Ok(LangIso::Kok),
            LangName::Korean => Ok(LangIso::Ko),
            LangName::Krio => Ok(LangIso::Kri),
            LangName::KurdishKurmanji => Ok(LangIso::Ku),
            LangName::KurdishSorani => Ok(LangIso::Ckb),
            LangName::Kyrgyz => Ok(LangIso::Ky),
            LangName::Lao => Ok(LangIso::Lo),
            LangName::Latin => Ok(LangIso::Lat),
            LangName::Latvian => Ok(LangIso::Lv),
            LangName::Lingala => Ok(LangIso::Ln),
            LangName::Lithuanian => Ok(LangIso::Lt),
            LangName::Luganda => Ok(LangIso::Lg),
            LangName::Luxembourgish => Ok(LangIso::Lb),
            LangName::Lacedonian => Ok(LangIso::Mk),
            LangName::Maithili => Ok(LangIso::Mai),
            LangName::Malagasy => Ok(LangIso::Mg),
            LangName::Malay => Ok(LangIso::Ms),
            LangName::Malayalam => Ok(LangIso::Ml),
            LangName::Maltese => Ok(LangIso::Mt),
            LangName::Maori => Ok(LangIso::Mi),
            LangName::Marathi => Ok(LangIso::Mr),
            LangName::Meiteilon => Ok(LangIso::Mni),
            LangName::Mizo => Ok(LangIso::Lus),
            LangName::Mongolian => Ok(LangIso::Mn),
            LangName::Myanmar => Ok(LangIso::My),
            LangName::Nepali => Ok(LangIso::Ne),
            LangName::Norwegian => Ok(LangIso::No),
            LangName::Odia => Ok(LangIso::Or),
            LangName::Oromo => Ok(LangIso::Om),
            LangName::Pashto => Ok(LangIso::Ps),
            LangName::Persian => Ok(LangIso::Fa),
            LangName::Polish => Ok(LangIso::Pol),
            LangName::Portuguese => Ok(LangIso::Pt),
            LangName::Punjabi => Ok(LangIso::Pa),
            LangName::Quechua => Ok(LangIso::Qwc),
            LangName::Romanian => Ok(LangIso::Ro),
            LangName::Russian => Ok(LangIso::Ru),
            LangName::Samoan => Ok(LangIso::Sm),
            LangName::Sanskrit => Ok(LangIso::Sa),
            LangName::ScotsGaelic => Ok(LangIso::Gd),
            LangName::Sepedi => Ok(LangIso::Nso),
            LangName::Serbian => Ok(LangIso::Sr),
            LangName::Sesotho => Ok(LangIso::St),
            LangName::Shona => Ok(LangIso::Sn),
            LangName::Sindhi => Ok(LangIso::Sd),
            LangName::Sinhala => Ok(LangIso::Si),
            LangName::Slovak => Ok(LangIso::Sk),
            LangName::Slovenian => Ok(LangIso::Sl),
            LangName::Somali => Ok(LangIso::So),
            LangName::Spanish => Ok(LangIso::Es),
            LangName::Sundanese => Ok(LangIso::Sun),
            LangName::Swahili => Ok(LangIso::Sw),
            LangName::Swedish => Ok(LangIso::Swe),
            LangName::Tajik => Ok(LangIso::Tg),
            LangName::Tamil => Ok(LangIso::Ta),
            LangName::Tatar => Ok(LangIso::Tat),
            LangName::Telugu => Ok(LangIso::Te),
            LangName::Thai => Ok(LangIso::Tha),
            LangName::Tigrinya => Ok(LangIso::Ti),
            LangName::Tsonga => Ok(LangIso::Ts),
            LangName::Turkish => Ok(LangIso::Tr),
            LangName::Turkmen => Ok(LangIso::Tk),
            LangName::Twi => Ok(LangIso::Tw),
            LangName::Ukrainian => Ok(LangIso::Uk),
            LangName::Urdu => Ok(LangIso::Ur),
            LangName::Uyghur => Ok(LangIso::Ug),
            LangName::Uzbek => Ok(LangIso::Uz),
            LangName::Vietnamese => Ok(LangIso::Vi),
            LangName::Welsh => Ok(LangIso::Cy),
            LangName::Xhosa => Ok(LangIso::Xh),
            LangName::Yiddish => Ok(LangIso::Yi),
            LangName::Yoruba => Ok(LangIso::Yo),
            LangName::Zulu => Ok(LangIso::Zu),
            _ => panic!("invalid language"),
        }
    }
}

/// Lang enum
/// The lang enum implements all translatable languages
#[derive(Debug, PartialEq, Eq, AsRefStr)]
pub enum Lang {
    Auto,
    Iso(LangIso),
    Name(LangName),
}

impl Lang {
    pub(crate) fn to_str(&self, variant: LangIso) -> &str {
        variant.as_ref()
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
/// use tl_lib::{translate, Lang}
///
/// fn main() -> ! {
///     let prompt = String::from("Hello world!")
///     let origin = Lang::Auto;
///     let target = Lang::German;
///     let translation = translate(prompt, origin, target);
///
///     match translation {
///         Err(e)  => eprintln!("{}", e),
///         Ok(res) => println!("{}", res) // Yields: Hallo welt!
///     }
/// }
///
pub fn translate(prompt: String, origin: Lang, target: Lang) -> Result<String, String> {
    let origin_lang = match origin {
        Lang::Auto => "auto",
        Lang::Iso(lang) => lang.as_ref(),
        Lang::Name(lang) => lang.to_iso().unwrap().as_ref(),
    };
    let origin_lang = map_lang(&origin);
    let mut target_lang = map_lang(&target);
    match parse(fetch(&prompt, &origin_lang, &target_lang)) {
        Ok(res) => Ok(res),
        Err(e) => Err(e),
    }
}
