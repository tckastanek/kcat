use errors::KcatError;
use std::path::Path;
use syntect::{
    dumps::{ from_dump_file, dump_to_file },
    highlighting::{ Theme, ThemeSet },
    parsing::SyntaxSet
};

pub fn get_theme_set(
    list_file_types: bool,
    list_embedded_themes: bool,
    syntax_set: &SyntaxSet
) -> ThemeSet {
    let ts = ThemeSet::load_defaults();
    
    if list_file_types {
        println!("Supported file types:");
        
        for sd in syntax_set.syntaxes() {
            println!("- {} (.{})", sd.name, sd.file_extensions.join(", ."));
        }
    } else if list_embedded_themes {
        println!("Embedded themes:");
        
        for t in ts.themes.keys() {
            println!("- {}", t);
        }
    }
    
    ts
}

pub fn get_syntax_set(no_default_syntaxes: bool, extra_syntaxes: Option<&str>) -> SyntaxSet {
    let mut ss: SyntaxSet = if no_default_syntaxes {
        SyntaxSet::new()
    } else {
        SyntaxSet::load_defaults_nonewlines()
    };
    
    if let Some(folder) = extra_syntaxes {
        ss.load_syntaxes(folder, false).unwrap();
        ss.link_syntaxes();
    }
    
    ss
}

pub fn load_theme(tm_file: &str, enable_caching: bool) -> Option<Theme> {
    let tm_path = Path::new(tm_file);
    
    if enable_caching {
        let tm_cache = tm_path.with_extension("tmdump");
        
        if tm_cache.exists() {
            from_dump_file(tm_cache).unwrap()
        } else {
            match ThemeSet::get_theme(tm_path) {
                Err(_) => {
                    println!("{}", KcatError::InvalidPath);
                    None
                },
                Ok(theme) => {
                    dump_to_file(&theme, tm_cache).unwrap();
                    Some(theme)
                }
            }
        }
    } else {
        match ThemeSet::get_theme(tm_path) {
            Err(_) => {
                println!("{}", KcatError::InvalidPath);
                None
            },
            Ok(theme) => Some(theme)
        }
    }
}
