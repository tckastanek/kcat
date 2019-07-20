extern crate serde_json;

use errors::KcatError;
use serde_json::Value;
use std::{fs::File, io::Read, path::Path};
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, Theme},
    parsing::SyntaxSet,
    util::as_24_bit_terminal_escaped,
};

pub fn print_fallback(path: &Path) {
    let mut s = String::new();
    match File::open(path) {
        Err(_) => println!("ERROR: {}", KcatError::InvalidPath),
        Ok(mut file) => {
            if file.read_to_string(&mut s).is_err() {
                println!("ERROR: {}", KcatError::InvalidFile);
            }
        }
    };

    println!("{}", s);
}

pub fn print_lines_with_extension(
    syntax_set: &SyntaxSet,
    theme: &Theme,
    path: &Path,
    extension: &str,
) {
    match syntax_set.find_syntax_by_extension(extension) {
        None => print_fallback(path),
        Some(syntax) => {
            let mut h = HighlightLines::new(syntax, theme);

            let mut s = String::new();
            match File::open(path) {
                Err(_) => println!("ERROR: {}", KcatError::InvalidPath),
                Ok(mut file) => match file.read_to_string(&mut s) {
                    Err(_) => println!("ERROR: {}", KcatError::InvalidFile),
                    Ok(_) => {
                        for line in s.lines() {
                            let ranges: Vec<(Style, &str)> = h.highlight(line, syntax_set);
                            let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
                            println!("{}", escaped);
                        }
                    }
                },
            }
        }
    }
}

pub fn print_json(
    syntax_set: &SyntaxSet,
    theme: &Theme,
    path: &Path,
    extension: &str,
    key_path: &str,
) {
    // should be safe as we only call this with extension as JSON
    let syntax = syntax_set.find_syntax_by_extension(extension).unwrap();
    let mut h = HighlightLines::new(syntax, &theme);
    let mut s = String::new();

    let keys = parse_key_path(key_path);
    match File::open(path) {
        Err(_) => println!("ERROR: {}", KcatError::InvalidPath),
        Ok(mut file) => match file.read_to_string(&mut s) {
            Err(_) => println!("ERROR: {}", KcatError::InvalidFile),
            Ok(_) => {
                let v: Value = serde_json::from_str(&s).unwrap();
                let final_value = get_value(&v, keys);
                if let Some(string) = serde_json::to_string_pretty(final_value).ok() {
                    for line in string.as_str().lines() {
                        let ranges: Vec<(Style, &str)> = h.highlight(line, syntax_set);
                        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
                        println!("{}", escaped);
                    }
                }
            }
        },
    }
}

// TODO: allow for optional delimiter argument
fn parse_key_path(key_path: &str) -> Vec<&str> {
    if key_path.starts_with('.') {
        let sliced = &key_path[1..];
        sliced.split('.').collect()
    } else {
        key_path.split('.').collect()
    }
}

fn get_value<'a>(value: &'a Value, mut keys: Vec<&str>) -> &'a Value {
    let key = keys[0];
    match value.get(key) {
        None => value,
        Some(next_value) => {
            keys.drain(0..1);
            if keys.len() == 0 {
                next_value
            } else {
                get_value(next_value, keys)
            }
        }
    }
}
