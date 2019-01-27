#[macro_use]
extern crate clap;
extern crate failure;
#[macro_use]
extern crate failure_derive;
extern crate serde_json;
extern crate syntect;

use clap::{App, Arg};
use constants::{
    CACHE_THEME_ARG, DEFAULT_THEME, EXTRA_SYNTAXES_ARG, FILE_ARG, KEY_PATH_ARG, LIST_THEMES_ARG,
    LIST_TYPES_ARG, NO_DEFAULT_SYNTAXES_ARG, THEME_FILE_ARG,
};
use printers::{print_fallback, print_json, print_lines_with_extension};
use std::{borrow::Cow, path::Path};
use theme_helpers::{get_syntax_set, get_theme_set, load_theme};

mod constants;
mod errors;
mod printers;
mod theme_helpers;

fn main() {
    let matches = App::new("Key Cat")
        .version(crate_version!())
        .author("Thomas Kastanek")
        .about("Like `cat`, but with key paths")
        .arg(Arg::with_name(FILE_ARG)
            .help("file path")
            .multiple(true)
            .value_name("FILE"))
        .arg(Arg::with_name(KEY_PATH_ARG)
            .short("k")
            .long(KEY_PATH_ARG)
            .help("key path")
            .value_name(KEY_PATH_ARG))
        .arg(Arg::with_name(NO_DEFAULT_SYNTAXES_ARG)
            .short("e")
            .long(NO_DEFAULT_SYNTAXES_ARG)
            .help("Doesn't load default syntaxes, intended for use with --extra-syntaxes.")
            .takes_value(false))
        .arg(Arg::with_name(EXTRA_SYNTAXES_ARG)
            .short("s")
            .long(EXTRA_SYNTAXES_ARG)
            .help("Additional folder to search for .sublime-syntax files in.")
            .value_name("SYNTAX FOLDER"))
        .arg(Arg::with_name(THEME_FILE_ARG)
            .short("t")
            .long(THEME_FILE_ARG)
            .help("Theme file to use. May be a path, or an embedded theme. Embedded themes will take precedence. Default: base16-ocean.dark")
            .value_name("THEME FILE"))
        .arg(Arg::with_name(LIST_TYPES_ARG)
            .short("l")
            .long(LIST_TYPES_ARG)
            .help("Lists supported file types")
            .takes_value(false))
        .arg(Arg::with_name(LIST_THEMES_ARG)
            .short("L")
            .long(LIST_THEMES_ARG)
            .help("Lists themes present in the executable")
            .takes_value(false))
        .arg(Arg::with_name(CACHE_THEME_ARG)
            .short("c")
            .long(CACHE_THEME_ARG)
            .help("Cache the parsed theme file")
            .takes_value(false))
        .get_matches();

    let no_default_syntaxes = matches.is_present(NO_DEFAULT_SYNTAXES_ARG);
    let extra_syntaxes = matches.value_of(EXTRA_SYNTAXES_ARG);
    let syntax_set = get_syntax_set(no_default_syntaxes, extra_syntaxes);

    let list_file_types = matches.is_present(LIST_TYPES_ARG);
    let list_embedded_themes = matches.is_present(LIST_THEMES_ARG);
    let theme_file = matches.value_of(THEME_FILE_ARG);
    let cache_theme = matches.is_present(CACHE_THEME_ARG);
    let theme_set = get_theme_set(list_file_types, list_embedded_themes, &syntax_set);
    let theme_file = theme_file.unwrap_or(DEFAULT_THEME);

    let loaded_theme = match theme_set.themes.get(theme_file).map(|t| Cow::Borrowed(t)) {
        None => match load_theme(theme_file, cache_theme) {
            None => None,
            Some(theme) => Some(Cow::Owned(theme)),
        },
        Some(theme) => Some(theme),
    };

    if let Some(theme) = loaded_theme {
        if matches.is_present(FILE_ARG) {
            for arg in matches.values_of(FILE_ARG).unwrap() {
                let file_path = Path::new(arg);
                match file_path.extension() {
                    None => print_fallback(&file_path),
                    Some(os_string) => match (os_string.to_str(), matches.value_of(KEY_PATH_ARG)) {
                        (Some(ext), Some(key_path)) if ext == "json" => {
                            print_json(&syntax_set, &theme, &file_path, &ext, &key_path);
                        }
                        (Some(ext), _) => {
                            print_lines_with_extension(&syntax_set, &theme, &file_path, &ext)
                        }
                        (_, _) => println!("Something went awry!"),
                    },
                };
            }
        }
    }

    // Clear the formatting
    println!("\x1b[0m");
}
