use cbindgen;
use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut config = cbindgen::Config::default();
    config.language = cbindgen::Language::C;
    config.cpp_compat = true;
    config.documentation = true;
    config.documentation_style = cbindgen::DocumentationStyle::Doxy;

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(config)
        .generate()
        .map_or_else(
            |error| match error {
                cbindgen::Error::ParseSyntaxError { .. } => {}
                e => panic!("{:?}", e),
            },
            |bindings| {
                bindings.write_to_file("target/include/translate.h");
            },
        );
}
