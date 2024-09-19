use crate::languages::{Config, Language};

use crossterm::style::{Attribute, Color};
use std::path::Path;
use tree_sitter::Node;
use tree_sitter_highlight::HighlightConfiguration;

crate::define_colors! {
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

pub fn tree_sitter_to_crossterm_color(index: usize, highlighter: &HighlightConfiguration, node: Node) -> (Color, Option<Attribute>) {
    if let Ok(theme) = crate::HIGHLIGHT_COLORS.read() {
        if !theme.is_empty() && index < theme.len() {
            return (theme[index], None);
        }
    }

    let lang = highlighter.language_name.to_owned();
    let name = highlighter.query.capture_names()[index];

    if lang == "toml" {
        match name {
            "string.special" => return (Colors::CYAN, None),
            "type" => return (Colors::AQUA, None),
            "variable.other.member" => return (Colors::MAGENTA, None),
            "constant.numeric.integer" | "constant.numeric.float" => return (Colors::YELLOW, None),
            _ => {}
        }
    }

    if lang == "html" || lang == "jsx" || lang == "xml" || lang == "tsx" {
        match name {
            "punctuation.special" => return (Colors::GREY, None),
            "variable.parameter" => return (Colors::MAGENTA, None),
            "tag" => return (Colors::GREEN, None),
            "constructor" => return (Colors::CYAN, None),
            _ => {}
        }

        match node.kind() {
            "\"" => return (Colors::ORANGE, None),
            "<" | ">" | "</" => return (Colors::DARK_GREEN, None),
            "identifier" => return (Colors::GREEN, None),
            "CData" => return (Color::White, None),
            "CDStart" | "]]>" => return (Colors::BLUE, None),
            "Name" | "attribute_name" => return (Colors::LIGHT_GREEN, None),
            "<?" | "?>" | "version" | "encoding" | "xml" => return (Colors::GREY, None),
            _ => {}
        }
    }

    match node.kind() {
        "escape_sequence" => (Colors::PEACH, Some(Attribute::Italic)),
        "identifier_reference" => (Colors::MAGENTA, None),
        "line_comment" | "js_comment" | "comment" => (Colors::DARK_GREY, None),
        "raw_text" => (Color::Grey, None),
        "attribute_name" | "word" => (Colors::MAGENTA, None),
        "tag_name" => (Colors::GREEN, None),
        "case" | "auto" => (Colors::AQUA, None),
        "#ifndef" | "#define" | "#include" => (Colors::GREY, Some(Attribute::Italic)),
        "null_scalar" => (Color::Grey, None),
        "regex_pattern" | "unit" | "@keyframes" => (Colors::YELLOW, None),
        "boolean" | "boolean_scalar" => (Colors::BLUE, None),
        "fenced_code_block" => (Colors::BLUE, None),
        "color_value" | "#" => (Colors::ORANGE, None),
        "code_fence_content" => (Color::Grey, None),
        "list_marker_minus" | "#{" => (Colors::GREY, None),
        "integer_literal" | "float_literal" | "thematic_break" | "list_marker_dot" | "integer_value" => (Colors::YELLOW, None),
        "mutable_specifier" => (Colors::CYAN, Some(Attribute::Italic)),
        _ => match name {
            "diff.minus" => (Colors::RED, None),
            "diff.plus" => (Colors::GREEN, None),
            "boolean" => (Colors::BLUE, None),
            "punctuation.special" | "text.title" => (Colors::ORANGE, None),
            "definition.module" => (Colors::BLUE, None),
            "property" => (Colors::MAGENTA, None),
            "function" | "function.call" => (Colors::GREEN, None),
            "function.macro" => (Colors::AQUA, Some(Attribute::Italic)),
            "function.method" | "constructor" | "method" => (Colors::CYAN, None),
            "keyword" | "keyword.operator" => (Colors::BLUE, None),
            "comment" => (Colors::DARK_GREY, None),
            "operator" | "attribute" | "punctuation.bracket" => (Colors::GREY, None),
            "string" | "string.special" | "comment.documentation" => (Colors::ORANGE, None),
            "variable.builtin" | "conditional" | "repeat" | "keyword.function" | "keyword.return" => (Colors::BLUE, None),
            "variable" | "variable.parameter" | "parameter" | "local.reference" => (Colors::MAGENTA, None),
            "number" | "float" => (Colors::YELLOW, None),
            "type" | "type.builtin" | "type.definition" => (Colors::CYAN, None),
            "type.enum.variant" => (Colors::BLUE, None),
            "constant" | "constant.builtin" => (Colors::BLUE, None),
            "punctuation" | "punctuation.delimiter" => (Color::White, None),
            "label" => (Color::Green, None),
            "module" | "include" => (Colors::BLUE, None),
            "error" => (Color::Red, None),
            _ => (Color::Reset, None),
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
        Some("asm" | "nasm") => "assembly",
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
