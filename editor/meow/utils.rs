use crate::languages::{Config, Language};
use crate::{color, crcolor, define_colors, italic};

use crossterm::style::{Attribute, Color};
use std::path::Path;
use tree_sitter::Node;
use tree_sitter_highlight::HighlightConfiguration;

define_colors! {
    RED => { r:255, g:0, b: 0 },
    GREY => { r:142, g:178, b:217 },
    CYAN => { r:48, g:232, b:233 },
    AQUA => { r:78, g:162, b:193 },
    BLUE => { r:103, g:179, b:255 },
    GREEN => { r: 45, g:232, b:170 },
    PEACH => { r:244, g:170, b:163 },
    YELLOW => { r:231, g:205, b:125 },
    ORANGE => { r:255, g:139, b:126 },
    MAGENTA => { r:205, g:162, b:244 },
    DARK_GREY => { r:80, g:85, b:89 },
    DARK_GREEN => { r:71, g:131, b:112 },
    LIGHT_GREEN => { r:164, g:225, b:133 },
}

pub fn convert_color(index: usize, highlighter: &HighlightConfiguration, node: Node) -> (Color, Option<Attribute>) {
    if let Ok(theme) = crate::HIGHLIGHT_COLORS.read() {
        if !theme.is_empty() && index < theme.len() {
            return (theme[index], None);
        }
    }

    let lang = highlighter.language_name.to_owned();
    let name = highlighter.query.capture_names()[index];

    if lang == "cpp" || lang == "c" {
        match name {
            "type" => return italic!(CYAN),
            "type.builtin" => return color!(BLUE),
            "keyword.control.conditional" => return color!(BLUE),
            _ => {}
        }
    }

    if lang == "yaml" {
        match name {
            "property" => return color!(MAGENTA),
            "punctuation.delimiter" | "punctuation.special" => return color!(GREY),
            "constant.numeric.integer" | "constant.numeric.float" => return color!(YELLOW),
            _ => {}
        }
    }

    if lang == "toml" {
        match name {
            "string.special" => return color!(CYAN),
            "type" => return color!(AQUA),
            "variable.other.member" => return color!(MAGENTA),
            "constant.numeric.integer" | "constant.numeric.float" => return color!(YELLOW),
            _ => {}
        }
    }

    if lang == "html" || lang == "jsx" || lang == "xml" || lang == "tsx" {
        match name {
            "punctuation.special" => return color!(GREY),
            "variable.parameter" => return color!(MAGENTA),
            "tag" => return color!(GREEN),
            "constructor" => return color!(CYAN),
            _ => {}
        }

        match node.kind() {
            "\"" => return color!(ORANGE),
            "<" | ">" | "</" => return color!(DARK_GREEN),
            "identifier" => return color!(GREEN),
            "CData" => return crcolor!(White),
            "CDStart" | "]]>" => return color!(BLUE),
            "Name" | "attribute_name" => return color!(LIGHT_GREEN),
            "<?" | "?>" | "version" | "encoding" | "xml" => return color!(GREY),
            _ => {}
        }
    }

    match node.kind() {
        "escape_sequence" => italic!(PEACH),
        "identifier_reference" => color!(MAGENTA),
        "line_comment" | "js_comment" | "comment" => color!(DARK_GREY),
        "raw_text" => crcolor!(Grey),
        "attribute_name" | "word" => color!(MAGENTA),
        "tag_name" => color!(GREEN),
        "case" | "auto" => color!(AQUA),
        "null_scalar" => crcolor!(Grey),
        "regex_pattern" | "unit" | "@keyframes" => color!(YELLOW),
        "boolean" | "boolean_scalar" => color!(BLUE),
        "fenced_code_block" => color!(BLUE),
        "color_value" | "#" => color!(ORANGE),
        "code_fence_content" => crcolor!(Grey),
        "list_marker_minus" | "#{" => color!(GREY),
        "integer_literal" | "float_literal" | "thematic_break" | "list_marker_dot" | "integer_value" => color!(YELLOW),
        "mutable_specifier" => italic!(CYAN),
        _ => match name {
            "keyword"
            | "keyword.operator"
            | "keyword.control"
            | "keyword.control.exception"
            | "keyword.control.return"
            | "keyword.control.conditional"
            | "keyword.control.repeat"
            | "keyword.storage.modifier"
            | "keyword.storage.type" => color!(BLUE),
            "keyword.directive" => italic!(GREY),
            "diff.minus" => color!(RED),
            "diff.plus" => color!(GREEN),
            "boolean" => color!(BLUE),
            "text.title" => color!(ORANGE),
            "definition.module" => color!(BLUE),
            "property" => color!(MAGENTA),
            "function" | "function.call" | "variable.other.member" => color!(GREEN),
            "function.macro" => italic!(AQUA),
            "function.method" | "constructor" | "method" => color!(CYAN),
            "comment" => color!(DARK_GREY),
            "operator" | "attribute" | "punctuation.bracket" | "punctuation.special" => color!(GREY),
            "string" | "string.special" | "comment.documentation" => color!(ORANGE),
            "variable.builtin" | "conditional" | "repeat" | "keyword.function" | "keyword.return" => color!(BLUE),
            "variable" | "variable.parameter" | "parameter" | "local.reference" => color!(MAGENTA),
            "number" | "float" | "constant.numeric" => color!(YELLOW),
            "type" | "type.builtin" | "type.definition" | "namespace" => color!(CYAN),
            "type.enum.variant" => color!(BLUE),
            "constant" | "constant.builtin" => color!(BLUE),
            "punctuation" | "punctuation.delimiter" => crcolor!(White),
            "label" => crcolor!(Green),
            "module" | "include" => color!(BLUE),
            "error" => crcolor!(Red),
            _ => crcolor!(Reset),
        },
    }
}

pub fn get_syntax(file_name: &Path) -> Option<Config> {
    if let Some(file_name_str) = file_name.file_name().and_then(|s| s.to_str()) {
        match file_name_str.to_lowercase().as_str() {
            "dockerfile" => return Some(Language::Dockerfile.config()),
            "makefile" => return Some(Language::Make.config()),
            "maidfile" => return Some(Language::Toml.config()),
            _ if file_name_str.starts_with('.') => match file_name_str.trim_start_matches('.') {
                "zshrc" | "bashrc" | "bash_profile" | "zprofile" | "gitignore" | "gitattributes" => return Some(Language::Bash.config()),
                _ => {}
            },
            _ => {}
        }
    }

    let file_name = match file_name.extension().and_then(|s| s.to_str()) {
        Some(name) => name,
        None => "",
    };

    match Language::from_token(file_name) {
        None => None,
        Some(lang) => Some(lang.config()),
    }
}

pub fn file_type(file_name: &Path) -> String {
    if let Some(file_name_str) = file_name.file_name().and_then(|s| s.to_str()) {
        match file_name_str.to_lowercase().as_str() {
            "dockerfile" => return "dockerfile".to_string(),
            "makefile" => return "makefile".to_string(),
            "maidfile" => return "maid".to_string(),
            "cmakelists.txt" => return "cmake".to_string(),
            _ if file_name_str.starts_with('.') => {
                let without_dot = file_name_str.trim_start_matches('.');
                match without_dot {
                    "zshrc" | "bashrc" | "bash_profile" | "zprofile" => return "shell".to_string(),
                    "gitignore" | "gitattributes" => return "git".to_string(),
                    _ => {}
                }
            }
            _ => {}
        }
    }

    match file_name.extension().and_then(|s| s.to_str()) {
        Some("abap") => "ABAP",
        Some("ada") => "ada",
        Some("ahk" | "ahkl") => "autohotkey",
        Some("applescript" | "scpt") => "applescript",
        Some("arc") => "Arc",
        Some("asp" | "asax" | "ascx" | "ashx" | "asmx" | "aspx" | "axd") => "ASP",
        Some("as") => "actionscript",
        Some("asc" | "ash") => "AGS",
        Some("asm" | "assembly" | "assembler" | "nasm" | "s") => "assembly",
        Some("awk" | "auk" | "gawk" | "mawk" | "nawk") => "awk",
        Some("bat" | "cmd") => "batch",
        Some("b" | "bf") => "brainfuck",
        Some("c") => "C",
        Some("cmake") => "cmake",
        Some("cbl" | "cobol" | "cob") => "cobol",
        Some("class" | "java") => "java",
        Some("clj" | "cl2" | "cljs" | "cljx" | "cljc") => "clojure",
        Some("coffee") => "coffeescript",
        Some("cr") => "crystal",
        Some("cu" | "cuh") => "cuda",
        Some("cpp" | "cxx" | "cc") => "C++",
        Some("cs" | "cshtml" | "csx") => "C#",
        Some("css") => "css",
        Some("csv") => "csv",
        Some("d" | "di") => "D",
        Some("dart") => "dart",
        Some("diff" | "patch") => "diff",
        Some("dockerfile") => "dockerfile",
        Some("ex" | "exs") => "elixr",
        Some("elm") => "elm",
        Some("el") => "emacs",
        Some("erb") => "ERB",
        Some("erl" | "es") => "frlang",
        Some("fs" | "fsi" | "fsx") => "F#",
        Some("f" | "f90" | "fpp" | "for") => "FORTRAN",
        Some("fish") => "fish",
        Some("fth") => "forth",
        Some("g4") => "ANTLR",
        Some("gd") => "gdscript",
        Some("glsl" | "vert" | "shader" | "geo" | "fshader" | "vrx" | "vsh" | "vshader" | "frag") => "GLSL",
        Some("gnu" | "gp" | "plot") => "gnuplot",
        Some("go") => "go",
        Some("groovy" | "gvy") => "groovy",
        Some("hlsl") => "HLSL",
        Some("h" | "hpp") => "header",
        Some("haml") => "haml",
        Some("handlebars" | "hbs") => "handlebars",
        Some("hs") => "haskell",
        Some("html" | "htm" | "xhtml") => "html",
        Some("ini" | "cfg") => "ini",
        Some("ino") => "arduino",
        Some("ijs") => "J",
        Some("json") => "json",
        Some("jsx") => "jsx",
        Some("js") => "javascript",
        Some("jl") => "julia",
        Some("kt" | "ktm" | "kts") => "kotlin",
        Some("ll") => "llvm",
        Some("less") => "less",
        Some("l" | "lex") => "lex",
        Some("lua") => "lua",
        Some("ls") => "livescript",
        Some("lol") => "lolcode",
        Some("lisp" | "asd" | "lsp") => "lisp",
        Some("log") => "logfile",
        Some("m4") => "M4",
        Some("man" | "roff") => "groff",
        Some("matlab") => "matlab",
        Some("m") => "objective-c",
        Some("ml") => "ocaml",
        Some("mk" | "mak") => "makefile",
        Some("md" | "markdown") => "markdown",
        Some("nix") => "nix",
        Some("numpy") => "numpy",
        Some("opencl" | "cl") => "opencl",
        Some("php") => "php",
        Some("pas") => "pascal",
        Some("pl") => "perl",
        Some("psl") => "powershell",
        Some("pro") => "prolog",
        Some("py" | "pyw") => "python",
        Some("pyx" | "pxd" | "pxi") => "cython",
        Some("r") => "R",
        Some("rst") => "reStructuredText",
        Some("rkt") => "racket",
        Some("rb" | "ruby") => "ruby",
        Some("rs") => "rust",
        Some("sh") => "shell",
        Some("scss") => "scss",
        Some("sql") => "sql",
        Some("sass") => "sass",
        Some("scala") => "scala",
        Some("sdml") => "SDML",
        Some("scm") => "scheme",
        Some("st") => "smalltalk",
        Some("swift") => "swift",
        Some("toml") => "toml",
        Some("tcl") => "tcl",
        Some("tex") => "TeX",
        Some("ts" | "tsx") => "typescript",
        Some("txt") => "plain",
        Some("vala") => "Vala",
        Some("vb" | "vbs") => "visual basic",
        Some("vue") => "Vue",
        Some("xm" | "x" | "xi") => "logos",
        Some("xml") => "XML",
        Some("y" | "yacc") => "yacc",
        Some("yaml" | "yml") => "yaml",
        Some("yxx") => "bison",
        Some("zsh") => "zsh",
        Some(other) => other,
        None => "unknown",
    }
    .to_string()
}

#[cfg(feature = "debugger")]
pub fn color_name(color: Color) -> String {
    match color {
        Color::Black => "#000000 (Black)".to_string(),
        Color::DarkGrey => "#555555 (DarkGrey)".to_string(),
        Color::Red => "#FF0000 (Red)".to_string(),
        Color::DarkRed => "#800000 (DarkRed)".to_string(),
        Color::Green => "#00FF00 (Green)".to_string(),
        Color::DarkGreen => "#008000 (DarkGreen)".to_string(),
        Color::Yellow => "#FFFF00 (Yellow)".to_string(),
        Color::DarkYellow => "#808000 (DarkYellow)".to_string(),
        Color::Blue => "#0000FF (Blue)".to_string(),
        Color::DarkBlue => "#000080 (DarkBlue)".to_string(),
        Color::Magenta => "#FF00FF (Magenta)".to_string(),
        Color::DarkMagenta => "#800080 (DarkMagenta)".to_string(),
        Color::Cyan => "#00FFFF (Cyan)".to_string(),
        Color::DarkCyan => "#008080 (DarkCyan)".to_string(),
        Color::White => "#FFFFFF (White)".to_string(),
        Color::Grey => "#808080 (Grey)".to_string(),
        Color::Rgb { r, g, b } => {
            let color_names = [
                ("Black", (0, 0, 0)),
                ("White", (255, 255, 255)),
                ("Red", (255, 0, 0)),
                ("Green", (0, 255, 0)),
                ("Blue", (0, 0, 255)),
                ("Yellow", (255, 255, 0)),
                ("Cyan", (0, 255, 255)),
                ("Magenta", (255, 0, 255)),
                ("Silver", (192, 192, 192)),
                ("Gray", (128, 128, 128)),
                ("Maroon", (128, 0, 0)),
                ("Olive", (128, 128, 0)),
                ("Dark Green", (0, 128, 0)),
                ("Purple", (128, 0, 128)),
                ("Teal", (0, 128, 128)),
                ("Navy", (0, 0, 128)),
                ("Orange", (255, 165, 0)),
                ("Pink", (255, 192, 203)),
                ("Brown", (165, 42, 42)),
                ("Coral", (255, 127, 80)),
                ("Tomato", (255, 99, 71)),
                ("Gold", (255, 215, 0)),
                ("Lavender", (230, 230, 250)),
                ("Light Yellow", (255, 255, 224)),
                ("Light Blue", (173, 216, 230)),
                ("Light Green", (144, 238, 144)),
                ("Light Pink", (255, 182, 193)),
                ("Light Cyan", (224, 255, 255)),
                ("Light Gray", (211, 211, 211)),
                ("Dark Orange", (255, 140, 0)),
                ("Indigo", (75, 0, 130)),
                ("Violet", (238, 130, 238)),
                ("Turquoise", (64, 224, 208)),
                ("Salmon", (250, 128, 114)),
                ("Khaki", (240, 230, 140)),
                ("Plum", (221, 160, 221)),
                ("Orchid", (218, 112, 214)),
                ("Sky Blue", (135, 206, 235)),
                ("Tan", (210, 180, 140)),
                ("Beige", (245, 245, 220)),
                ("Mint", (189, 252, 201)),
                ("Slate Gray", (112, 128, 144)),
                ("Forest Green", (34, 139, 34)),
                ("Crimson", (220, 20, 60)),
            ];

            let closest_color = color_names
                .iter()
                .min_by_key(|&&(_, (cr, cg, cb))| {
                    let dr = r as i32 - cr as i32;
                    let dg = g as i32 - cg as i32;
                    let db = b as i32 - cb as i32;
                    (2 * dr * dr + 4 * dg * dg + 3 * db * db) as u32
                })
                .map(|&(name, _)| name)
                .unwrap_or("Unknown");

            format!("#{:02X}{:02X}{:02X} ({})", r, g, b, closest_color)
        }
        _ => "Unsupported color type".to_string(),
    }
}
