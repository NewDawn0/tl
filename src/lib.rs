use html_escape::decode_html_entities;
use reqwest::blocking::get;
use std::ffi::{c_char, CStr, CString};
use strum::EnumIter;

/// An enumeration representing different languages supported for translation.
/// Each variant corresponds to a language that can be translated from and to.
#[repr(C)]
#[derive(EnumIter)]
pub enum Language {
    /// Automatic translation (language will be detected automatically).
    Auto,
    /// Afrikaans language.
    Afrikaans,
    /// Albanian language.
    Albanian,
    /// Amharic language.
    Amharic,
    /// Arabic language.
    Arabic,
    /// Armenian language.
    Armenian,
    /// Azerbaijani language.
    Azerbaijani,
    /// Basque language.
    Basque,
    /// Belarusian language.
    Belarusian,
    /// Bengali language.
    Bengali,
    /// Bosnian language.
    Bosnian,
    /// Bulgarian language.
    Bulgarian,
    /// Catalan language.
    Catalan,
    /// Cebuano language.
    Cebuano,
    /// Chinese (Simplified) language.
    ChineseSimplified,
    /// Chinese (Traditional) language.
    ChineseTraditional,
    /// Croatian language.
    Croatian,
    /// Czech language.
    Czech,
    /// Danish language.
    Danish,
    /// Dutch language.
    Dutch,
    /// English language.
    English,
    /// Estonian language.
    Estonian,
    /// Filipino language.
    Filipino,
    /// Finnish language.
    Finnish,
    /// French language.
    French,
    /// Georgian language.
    Georgian,
    /// German language.
    German,
    /// Greek language.
    Greek,
    /// Gujarati language.
    Gujarati,
    /// Hebrew language.
    Hebrew,
    /// Hindi language.
    Hindi,
    /// Hungarian language.
    Hungarian,
    /// Icelandic language.
    Icelandic,
    /// Indonesian language.
    Indonesian,
    /// Irish language.
    Irish,
    /// Italian language.
    Italian,
    /// Japanese language.
    Japanese,
    /// Kannada language.
    Kannada,
    /// Kazakh language.
    Kazakh,
    /// Khmer language.
    Khmer,
    /// Korean language.
    Korean,
    /// Kurdish language.
    Kurdish,
    /// Lao language.
    Lao,
    /// Latvian language.
    Latvian,
    /// Lithuanian language.
    Lithuanian,
    /// Macedonian language.
    Macedonian,
    /// Malay language.
    Malay,
    /// Malayalam language.
    Malayalam,
    /// Marathi language.
    Marathi,
    /// Mongolian language.
    Mongolian,
    /// Nepali language.
    Nepali,
    /// Norwegian language.
    Norwegian,
    /// Persian language.
    Persian,
    /// Polish language.
    Polish,
    /// Portuguese language.
    Portuguese,
    /// Romanian language.
    Romanian,
    /// Russian language.
    Russian,
    /// Serbian language.
    Serbian,
    /// Sinhala language.
    Sinhala,
    /// Slovak language.
    Slovak,
    /// Slovenian language.
    Slovenian,
    /// Spanish language.
    Spanish,
    /// Swedish language.
    Swedish,
    /// Tamil language.
    Tamil,
    /// Telugu language.
    Telugu,
    /// Thai language.
    Thai,
    /// Turkish language.
    Turkish,
    /// Ukrainian language.
    Ukrainian,
    /// Urdu language.
    Urdu,
    /// Vietnamese language.
    Vietnamese,
    /// Welsh language.
    Welsh,
    /// Xhosa language.
    Xhosa,
    /// Yiddish language.
    Yiddish,
    /// Yoruba language.
    Yoruba,
    /// Zulu language.
    Zulu,
}

impl Language {
    /// Converts a `Language` enum variant into a tuple representing the
    /// language's code and name.
    ///
    /// # Returns
    ///
    /// A tuple containing two strings:
    /// 1. A string representing the language code (e.g., "en" for English).
    /// 2. A string representing the name of the language (e.g., "english").
    pub const fn to_str(&self) -> (&str, &str) {
        match self {
            Self::Auto => ("auto", "auto"),
            Self::Afrikaans => ("af", "afrikaans"),
            Self::Albanian => ("sq", "albanian"),
            Self::Amharic => ("am", "amharic"),
            Self::Arabic => ("ar", "arabic"),
            Self::Armenian => ("hy", "armenian"),
            Self::Azerbaijani => ("az", "azerbaijani"),
            Self::Basque => ("eu", "basque"),
            Self::Belarusian => ("be", "belarusian"),
            Self::Bengali => ("bn", "bengali"),
            Self::Bosnian => ("bs", "bosnian"),
            Self::Bulgarian => ("bg", "bulgarian"),
            Self::Catalan => ("ca", "catalan"),
            Self::Cebuano => ("ceb", "cebuano"),
            Self::ChineseSimplified => ("zh-CN", "chinese_simplified"),
            Self::ChineseTraditional => ("zh-TW", "chinese_traditional"),
            Self::Croatian => ("hr", "croatian"),
            Self::Czech => ("cs", "czech"),
            Self::Danish => ("da", "danish"),
            Self::Dutch => ("nl", "dutch"),
            Self::English => ("en", "english"),
            Self::Estonian => ("et", "estonian"),
            Self::Filipino => ("tl", "filipino"),
            Self::Finnish => ("fi", "finnish"),
            Self::French => ("fr", "french"),
            Self::Georgian => ("ka", "georgian"),
            Self::German => ("de", "german"),
            Self::Greek => ("el", "greek"),
            Self::Gujarati => ("gu", "gujarati"),
            Self::Hebrew => ("he", "hebrew"),
            Self::Hindi => ("hi", "hindi"),
            Self::Hungarian => ("hu", "hungarian"),
            Self::Icelandic => ("is", "icelandic"),
            Self::Indonesian => ("id", "indonesian"),
            Self::Irish => ("ga", "irish"),
            Self::Italian => ("it", "italian"),
            Self::Japanese => ("ja", "japanese"),
            Self::Kannada => ("kn", "kannada"),
            Self::Kazakh => ("kk", "kazakh"),
            Self::Khmer => ("km", "khmer"),
            Self::Korean => ("ko", "korean"),
            Self::Kurdish => ("ku", "kurdish"),
            Self::Lao => ("lo", "lao"),
            Self::Latvian => ("lv", "latvian"),
            Self::Lithuanian => ("lt", "lithuanian"),
            Self::Macedonian => ("mk", "macedonian"),
            Self::Malay => ("ms", "malay"),
            Self::Malayalam => ("ml", "malayalam"),
            Self::Marathi => ("mr", "marathi"),
            Self::Mongolian => ("mn", "mongolian"),
            Self::Nepali => ("ne", "nepali"),
            Self::Norwegian => ("no", "norwegian"),
            Self::Persian => ("fa", "persian"),
            Self::Polish => ("pl", "polish"),
            Self::Portuguese => ("pt", "portuguese"),
            Self::Romanian => ("ro", "romanian"),
            Self::Russian => ("ru", "russian"),
            Self::Serbian => ("sr", "serbian"),
            Self::Sinhala => ("si", "sinhala"),
            Self::Slovak => ("sk", "slovak"),
            Self::Slovenian => ("sl", "slovenian"),
            Self::Spanish => ("es", "spanish"),
            Self::Swedish => ("sv", "swedish"),
            Self::Tamil => ("ta", "tamil"),
            Self::Telugu => ("te", "telugu"),
            Self::Thai => ("th", "thai"),
            Self::Turkish => ("tr", "turkish"),
            Self::Ukrainian => ("uk", "ukrainian"),
            Self::Urdu => ("ur", "urdu"),
            Self::Vietnamese => ("vi", "vietnamese"),
            Self::Welsh => ("cy", "welsh"),
            Self::Xhosa => ("xh", "xhosa"),
            Self::Yiddish => ("yi", "yiddish"),
            Self::Yoruba => ("yo", "yoruba"),
            Self::Zulu => ("zu", "zulu"),
        }
    }

    /// Converts a string representation of a language to the corresponding
    /// `Language` enum variant.
    ///
    /// # Parameters
    ///
    /// - `lang`: A string representing the language code or name (e.g., "en" or "english").
    ///
    /// # Returns
    ///
    /// An `Option` containing the corresponding `Language` variant if found,
    /// or `None` if the string does not match any valid language.
    pub fn from_str(lang: &str) -> Option<Self> {
        match lang {
            "auto" => Some(Self::Auto),
            "af" | "afrikaans" => Some(Self::Afrikaans),
            "sq" | "albanian" => Some(Self::Albanian),
            "am" | "amharic" => Some(Self::Amharic),
            "ar" | "arabic" => Some(Self::Arabic),
            "hy" | "armenian" => Some(Self::Armenian),
            "az" | "azerbaijani" => Some(Self::Azerbaijani),
            "eu" | "basque" => Some(Self::Basque),
            "be" | "belarusian" => Some(Self::Belarusian),
            "bn" | "bengali" => Some(Self::Bengali),
            "bs" | "bosnian" => Some(Self::Bosnian),
            "bg" | "bulgarian" => Some(Self::Bulgarian),
            "ca" | "catalan" => Some(Self::Catalan),
            "ceb" | "cebuano" => Some(Self::Cebuano),
            "zh-CN" | "chinese_simplified" => Some(Self::ChineseSimplified),
            "zh-TW" | "chinese_traditional" => Some(Self::ChineseTraditional),
            "hr" | "croatian" => Some(Self::Croatian),
            "cs" | "czech" => Some(Self::Czech),
            "da" | "danish" => Some(Self::Danish),
            "nl" | "dutch" => Some(Self::Dutch),
            "en" | "english" => Some(Self::English),
            "et" | "estonian" => Some(Self::Estonian),
            "tl" | "filipino" => Some(Self::Filipino),
            "fi" | "finnish" => Some(Self::Finnish),
            "fr" | "french" => Some(Self::French),
            "ka" | "georgian" => Some(Self::Georgian),
            "de" | "german" => Some(Self::German),
            "el" | "greek" => Some(Self::Greek),
            "gu" | "gujarati" => Some(Self::Gujarati),
            "he" | "hebrew" => Some(Self::Hebrew),
            "hi" | "hindi" => Some(Self::Hindi),
            "hu" | "hungarian" => Some(Self::Hungarian),
            "is" | "icelandic" => Some(Self::Icelandic),
            "id" | "indonesian" => Some(Self::Indonesian),
            "ga" | "irish" => Some(Self::Irish),
            "it" | "italian" => Some(Self::Italian),
            "ja" | "japanese" => Some(Self::Japanese),
            "kn" | "kannada" => Some(Self::Kannada),
            "kk" | "kazakh" => Some(Self::Kazakh),
            "km" | "khmer" => Some(Self::Khmer),
            "ko" | "korean" => Some(Self::Korean),
            "ku" | "kurdish" => Some(Self::Kurdish),
            "lo" | "lao" => Some(Self::Lao),
            "lv" | "latvian" => Some(Self::Latvian),
            "lt" | "lithuanian" => Some(Self::Lithuanian),
            "mk" | "macedonian" => Some(Self::Macedonian),
            "ms" | "malay" => Some(Self::Malay),
            "ml" | "malayalam" => Some(Self::Malayalam),
            "mr" | "marathi" => Some(Self::Marathi),
            "mn" | "mongolian" => Some(Self::Mongolian),
            "ne" | "nepali" => Some(Self::Nepali),
            "no" | "norwegian" => Some(Self::Norwegian),
            "fa" | "persian" => Some(Self::Persian),
            "pl" | "polish" => Some(Self::Polish),
            "pt" | "portuguese" => Some(Self::Portuguese),
            "ro" | "romanian" => Some(Self::Romanian),
            "ru" | "russian" => Some(Self::Russian),
            "sr" | "serbian" => Some(Self::Serbian),
            "si" | "sinhala" => Some(Self::Sinhala),
            "sk" | "slovak" => Some(Self::Slovak),
            "sl" | "slovenian" => Some(Self::Slovenian),
            "es" | "spanish" => Some(Self::Spanish),
            "sv" | "swedish" => Some(Self::Swedish),
            "ta" | "tamil" => Some(Self::Tamil),
            "te" | "telugu" => Some(Self::Telugu),
            "th" | "thai" => Some(Self::Thai),
            "tr" | "turkish" => Some(Self::Turkish),
            "uk" | "ukrainian" => Some(Self::Ukrainian),
            "ur" | "urdu" => Some(Self::Urdu),
            "vi" | "vietnamese" => Some(Self::Vietnamese),
            "cy" | "welsh" => Some(Self::Welsh),
            "xh" | "xhosa" => Some(Self::Xhosa),
            "yi" | "yiddish" => Some(Self::Yiddish),
            "yo" | "yoruba" => Some(Self::Yoruba),
            "zu" | "zulu" => Some(Self::Zulu),
            _ => None,
        }
    }
}

/// Translates a given text from one language to another.
///
/// This function sends a request to a translation service (e.g., Google Translate)
/// and retrieves the translated text.
///
/// # Parameters
/// - `text`: The text that should be translated.
/// - `from`: The source language (`Language` enum).
/// - `to`: The target language (`Language` enum).
///
/// # Returns
/// A `Result` containing the translated text on success, or an error message on failure.
/// cbindgen:no-export
pub fn translate(text: &str, from: Language, to: Language) -> Result<String, String> {
    parse(fetch(text, from, to))
}

/// C-compatible wrapper function for the `translate` function.
///
/// This function is intended to be called from C code. It converts the input
/// C string to a Rust string, performs the translation, and returns the result
/// as a C string.
///
/// # Parameters
/// - `text`: A pointer to a null-terminated C string containing the text to be translated.
/// - `from`: The source language (`Language` enum).
/// - `to`: The target language (`Language` enum).
///
/// # Returns
/// A pointer to a null-terminated C string containing the translated text, or
/// an error message if the translation fails.
#[no_mangle]
pub extern "C" fn c_translate(text: *const c_char, from: Language, to: Language) -> *mut c_char {
    let text = unsafe { CStr::from_ptr(text).to_str().unwrap_or("") };
    match parse(fetch(text, from, to)) {
        Ok(result) => {
            let c_result = CString::new(result).unwrap();
            c_result.into_raw()
        }
        Err(_) => {
            let err_msg = CString::new("Translation failed").unwrap();
            err_msg.into_raw()
        }
    }
}

/// Frees a C-style string previously allocated by Rust and returned over FFI.
///
/// # Safety
///
/// This function **must** only be called with pointers returned from Rust
/// (e.g., a `CString::into_raw()` result). Passing any other pointer,
/// a pointer that was already freed, or a null pointer will result in
/// undefined behavior, memory corruption, or a crash.
///
/// # Parameters
///
/// * `s` - A `*mut c_char` pointer to a C-style string. If the pointer is null, the function does nothing.
///
/// # Example (C-side usage)
///
/// ```c
/// char* translated = c_translate(...);
/// // use `translated`
/// c_translate_free(translated); // free the memory when done
/// ```
///
/// # Note
///
/// This function is intended to be used with FFI (Foreign Function Interface)
/// to safely deallocate memory passed from Rust to C.
///
/// The `#[no_mangle]` and `extern "C"` annotations ensure the function can
/// be called from C code without name mangling.
///
/// # FFI-Compatible
///
/// This function is FFI-safe and callable from other languages that can interface with C.
#[no_mangle]
pub extern "C" fn c_translate_free(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s);
    }
}

/// Fetches translation data from a translation service (e.g., Google Translate).
///
/// This function constructs a URL for the translation request and sends a
/// blocking HTTP request to fetch the translation.
///
/// # Parameters
/// - `text`: The text that should be translated.
/// - `from`: The source language (`Language` enum).
/// - `to`: The target language (`Language` enum).
///
/// # Returns
/// A `Result` containing the response body on success, or an error message on failure.
fn fetch(text: &str, from: Language, to: Language) -> Result<String, String> {
    let url = format!(
        "https://translate.google.com/m?tl={}&sl={}&q={}",
        to.to_str().0,
        from.to_str().0,
        text
    );
    match get(url) {
        Err(resp) => Err(resp.to_string()),
        Ok(resp) => match resp.text() {
            Ok(body) => Ok(body),
            Err(err) => Err(err.to_string()),
        },
    }
}

/// Parses the HTML response from the translation service and extracts the translated text.
///
/// # Arguments
///
/// * `res` - A `Result<String, String>` containing either the raw HTML response (`Ok`) or an error message (`Err`).
///
/// # Returns
///
/// A `Result<String, String>`:
/// - `Ok(String)` with the translated text if successful.
/// - `Err(String)` with an error message if parsing fails or if the input was already an error.
///
/// # Behavior
///
/// - If `res` is `Err`, the function returns it immediately.
/// - If `res` is `Ok`, it attempts to parse the HTML and extract the first element with the class `result-container`.
/// - If the element is not found, it returns an error message "Error decoding".
/// - Otherwise, it decodes any HTML entities from the inner text and returns it.
///
/// # Example
///
/// ```
/// let html = r#"<div class="result-container">Hello &amp; welcome</div>"#.to_string();
/// let result = parse(Ok(html));
/// assert_eq!(result.unwrap(), "Hello & welcome");
/// ```
fn parse(res: Result<String, String>) -> Result<String, String> {
    match res {
        Err(err) => Err(err),
        Ok(body) => match tl::parse(&body).get_elements_by_class_name("result-container") {
            None => Err(String::from("Error decoding")),
            Some(e) => Ok(decode_html_entities(&e[0].inner_text()).to_string()),
        },
    }
}
