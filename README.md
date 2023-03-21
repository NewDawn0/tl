# TL - TransLator
**A simple translator which is written in Rust**

## Usage and examples
Basic usage
```bash
tl <options> phrase
```
To set the target language, simply add the `-t` or `--to` flag and either the language ISO code or the language name to the arguments. Both the language ISO codes and names can be shown when running `tl --help` and scrolling down to the language section
```bash
tl This phrase will be translated to japanese -t ja
```
Since tl accepts both ISO codes and language names the below example works as well.
```bash
tl This phrase will be translated to japanese -t japanese
```
To set the origin language, simply provide the "-f" or "--from" flag and again either the language ISO code or name.
Note that to translate to English we don't have to set the target language as it is set to English by default nor do we have to set the origin language as it is set to Google's automatic recognition by default

Arguments like the origin and the target language can be provided anywhere in the arguments, they don't have to specifically be at the beginning or end, whether that's a good or a bad thing is up to you to decide

## Installation
### Binary package
To install the binary version run
```bash
cargo install --git https://github.com/NewDawn0/tl.git
```
### Add it as a library to your project
To add the ability to translate words/sentences to your rust project, run
```bash
cargo add --git https://github.com/NewDawn0/tl.git
```

## Program description courtesy of the built-in --help flag
```bash
NAME
    TL (by NewDawn0)
    TransLate - A tool to translate sentences

SYNOPSIS
    tl [-h | --help] [<language or language code> <string(s)> | <language or language code> <string(s)>]

DESCRIPTION
    A Tool that translates text using goole translate

OPTIONS
    -h             opens this screen
    --help         opens this screen
    -f <lang>      set the origin language (defaults to auto if not provided)
    --from <lang>  set the origin language (defaults to auto if not provided)
    -t <lang>      set the target language (defaults to english if not provided)
    -to <lang>     set the target language (defaults to english if not provided)

EXAMPLE
    $ tl TL - Rust で NeoVim を使用して NewDawn0 によって作成されました
    > TL - Created by NewDawn0 using NeoVim in Rust

LANGUAGES
    A:  af - afrikaans;  sq - albanian; am - amharic; ar - arabic;
        hy - armenian; as - assamese; ay - aymara
    B:  bm - bambara; eu - basque; be - belarusian; bn - bengali
        bho - bhojpuri; bs - bosnian; bg - bulgarian
    C:  ca - catalan; ceb - cebuano; ny - chichewa;
        zh-CN - chinese simplified; zh-TW - chinese traditional;
        co - corsican; hr - croatian; cs - czech
    D:  da - danish; dv - dhivehi; doi - dogri; nl - dutch
    E:  en - english; eo - esperanto; et - estonian; ee - ewe
    F:  fil - filipino; fi - finnish; fr - french; fy - frisian
    G:  gl - galician; ka - georgian; de - german; el - greek;
        gn - guarani; gu - gujarati
    H:  ht - haitian; ha - hausa; haw - hawaiian; he - hebrew;
        hi - hindi; hmn -  hmong; hun -  hungarian
    I:  isl - icelandic; ig - igbo; ilo - ilocano;
        in - indonesian; ga - irish; it - italian
    J:  ja - japanese; jv -  javanese
    K:  kn - kannada; kk - kazakh; km - khmer; rw - kinyarwanda;
        kok - konkani; ko - korean; kri - krio; ku - kurdish kurmanji;
        ckb - kurdish sorani; ky - kyrgyz
    L:  lo - lao; lat - latin; lv - latvian; ln - lingala;
        lt - lithuanian; lg - luganda; lb - luxembourgish
    M:  mk - macedonian; mai - maithili; mg - malagasy; ms - malay;
        ml - malayalam; mt - maltese; mi - maori; mr - marathi;
        mni - meiteilon; lus - mizo; mn - mongolian; my - myanmar
    N:  ne - nepali; no - norwegian
    O:  or - odia; om - oromo
    P:  ps - pashto; fa - persian; pol - polish
        pt - portuguese; pa - punjabi
    Q:  qwc - quechua
    R:  ro - romanian; ru - russian
    J:  sm - samoan; sa - sanskrit; gd - scots gaelic;
    S:  nso - sepedi; sr - serbian; st - sesotho; sn - shona;
        sd - sindhi; si - sinhala; sk - slovak; sl - slovenian;
        so - somali; es - spanish; sun - sundanese; sw - swahili;
        swe - swedish
    T:  tg - tajik; ta - tamil; tat -   tatar; te - telugu; tha - thai
        ti - tigrinya; ts - tsonga; tr - turkish; tk - turkmen; tw - twi
    U:  uk - ukrainian; ur - urdu; ug - uyghur; uz - uzbek
    V:  vi - vietnamese
    W:  cy - welsh
    X:  xh - xhosa
    Y:  yi - yiddish; yo - yoruba
    Z:  zu - zulu;
```

## Changelog
- **0v.1.7** Added library API
- **v0.1.6** Escaped all encodings implicitly
- **v0.1.5** Fixed apostrophe bug in French
- **v0.1.4** Fixed argument parser
- **v0.1.3** Added new language selection mode
- **v0.1.2** Added an improved argument parser
- **v0.1.1** Minor Bugfixes
- **v0.1.0** Added initial translator
