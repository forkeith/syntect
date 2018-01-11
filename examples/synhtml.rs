//! Prints highlighted HTML for a file to stdout.
//! Basically just wraps a body around `highlighted_snippet_for_file`
extern crate syntect;
extern crate getopts;

use getopts::Options;
use std::borrow::Cow;
use std::path::Path;
use syntect::parsing::SyntaxSet;
use syntect::highlighting::{Color, ThemeSet, Theme};
use syntect::html::highlighted_snippet_for_file;
use syntect::dumps::{from_dump_file, dump_to_file};

fn load_theme(tm_file: &String, enable_caching: bool) -> Theme {
    let tm_path = Path::new(tm_file);

    if enable_caching {
        let tm_cache = tm_path.with_extension("tmdump");

        if tm_cache.exists() {
            from_dump_file(tm_cache).unwrap()
        } else {
            let theme = ThemeSet::get_theme(tm_path).unwrap();
            dump_to_file(&theme, tm_cache).unwrap();
            theme
        }
    } else {
        ThemeSet::get_theme(tm_path).unwrap()
    }
}

fn main() {

    let args: Vec<String> = std::env::args().collect();
    let mut opts = Options::new();
    opts.optflag("l", "list-file-types", "Lists supported file types");
    opts.optflag("L", "list-embedded-themes", "Lists themes present in the executable");
    opts.optopt("t", "theme-file", "THEME_FILE", "Theme file to use. May be a path, or an embedded theme. Embedded themes will take precendence. Default: base16-ocean.dark");
    opts.optopt("s", "extra-syntaxes", "SYNTAX_FOLDER", "Additional folder to search for .sublime-syntax files in.");
    opts.optflag("e", "no-default-syntaxes", "Doesn't load default syntaxes, intended for use with --extra-syntaxes.");
    opts.optflag("c", "cache-theme", "Cache the parsed theme file.");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let mut ss = if matches.opt_present("no-default-syntaxes") {
        SyntaxSet::new()
    } else {
        SyntaxSet::load_defaults_newlines()
    };

    if let Some(folder) = matches.opt_str("extra-syntaxes") {
        ss.load_syntaxes(folder, true).unwrap();
        ss.link_syntaxes();
    }

    let ts = ThemeSet::load_defaults();

    if matches.opt_present("list-file-types") {
        println!("Supported file types:");

        for sd in ss.syntaxes() {
            println!("- {} (.{})", sd.name, sd.file_extensions.join(", ."));
        }

    } else if matches.opt_present("list-embedded-themes") {
        println!("Embedded themes:");

        for t in ts.themes.keys() {
            println!("- {}", t);
        }

    } else if matches.free.len() == 0 {
        let brief = format!("USAGE: {} [options] FILES", args[0]);
        println!("{}", opts.usage(&brief));

    } else {
        let theme_file : String = matches.opt_str("theme-file")
            .unwrap_or("base16-ocean.dark".to_string());

        let theme = ts.themes.get(&theme_file)
            .map(|t| Cow::Borrowed(t))
            .unwrap_or_else(|| Cow::Owned(load_theme(&theme_file, matches.opt_present("cache-theme"))));

        let style = "
            pre {
                font-size:13px;
                font-family: Consolas, \"Liberation Mono\", Menlo, Courier, monospace;
            }";
        println!("<head><title>{}</title><style>{}</style></head>", &matches.free[0], style);
        let c = theme.settings.background.unwrap_or(Color::WHITE);
        println!("<body style=\"background-color:#{:02x}{:02x}{:02x};\">\n", c.r, c.g, c.b);
        for src in &matches.free[..] {
            if matches.free.len() > 1 {
                println!("<p>==> {} <==</p>", src);
            }

            let html = highlighted_snippet_for_file(src, &ss, &theme).unwrap();
            println!("{}", html);
        }
        println!("</body>");
    }
}
