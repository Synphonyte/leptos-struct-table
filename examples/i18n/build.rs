use leptos_i18n_build::{Config, TranslationsInfos};
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=Cargo.toml");

    // where to generate the translations
    let i18n_mod_directory = PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("i18n");

    let cfg = Config::new("en")?.add_locale("de")?;

    let translations_infos = TranslationsInfos::parse(cfg)?;

    // emit the errors and warnings found during parsing
    translations_infos.emit_diagnostics();

    // emit "cargo::rerun-if-changed" for every translation file
    translations_infos.rerun_if_locales_changed();

    // codegen
    translations_infos.generate_i18n_module(i18n_mod_directory)?;

    Ok(())
}
