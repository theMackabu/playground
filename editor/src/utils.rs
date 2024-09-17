use crate::define_colors;
use const_format::concatcp;
use crossterm::style::{Attribute, Color};
use std::path::Path;
use tree_sitter::{Language, Node};

pub fn tree_sitter_to_crossterm_color(highlight_name: &str, lang: &str, node: Node) -> (Color, Option<Attribute>) {
    define_colors! {
        GREY => { r:142, g:178, b:217 },
        CYAN => { r:48, g:232, b:233 },
        AQUA => { r:78, g:162, b:193 },
        BLUE => { r:103, g:179, b:255 },
        GREEN => { r:45, g:232, b:170 },
        YELLOW => { r:231, g:205, b:125 },
        ORANGE => { r:255, g:139, b:126 },
        MAGENTA => { r:205, g:162, b:244 },
        DARK_GREEN => { r:71, g:131, b:112 },
    };

    if lang == "html" || lang == "jsx" {
        match node.kind() {
            "<" | ">" | "</" => return (Colors::DARK_GREEN, None),
            _ => {}
        }

        if highlight_name == "tag" {
            return (Colors::DARK_GREEN, None);
        }
    }

    match node.kind() {
        "line_comment" | "js_comment" => (Color::DarkGrey, None),
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
        "list_marker_minus" => (Colors::GREY, None),
        "integer_literal" | "float_literal" | "thematic_break" | "list_marker_dot" | "integer_value" => (Colors::YELLOW, None),
        "mutable_specifier" => (Colors::CYAN, Some(Attribute::Italic)),
        _ => match highlight_name {
            "boolean" => (Colors::BLUE, None),
            "punctuation.special" | "text.title" => (Colors::ORANGE, None),
            "definition.module" => (Colors::BLUE, None),
            "property" => (Colors::MAGENTA, None),
            "function" => (Colors::GREEN, None),
            "function.macro" => (Colors::AQUA, Some(Attribute::Italic)),
            "function.method" | "constructor" => (Colors::CYAN, None),
            "keyword" | "keyword.operator" => (Colors::BLUE, None),
            "comment" => (Color::DarkGrey, None),
            "operator" | "attribute" | "punctuation.bracket" => (Colors::GREY, None),
            "string" | "string.special" | "comment.documentation" => (Colors::ORANGE, None),
            "variable.builtin" | "conditional" | "repeat" | "keyword.function" | "keyword.return" => (Colors::BLUE, None),
            "variable" | "variable.parameter" => (Colors::MAGENTA, None),
            "number" | "float" => (Colors::YELLOW, None),
            "type" | "type.builtin" => (Colors::CYAN, None),
            "type.enum.variant" => (Colors::BLUE, None),
            "constant" | "constant.builtin" => (Colors::BLUE, None),
            "punctuation" | "punctuation.delimiter" => (Color::White, None),
            "label" => (Color::Green, None),
            "module" => (Colors::BLUE, None),
            "error" => (Color::Red, None),
            _ => (Color::Reset, None),
        },
    }
}

pub fn get_syntax(file_name: &Path) -> Option<(Language, (&'static str, &'static str, &'static str), &'static str)> {
    if let Some(file_name_str) = file_name.file_name().and_then(|s| s.to_str()) {
        match file_name_str.to_lowercase().as_str() {
            "dockerfile" => return Some((tree_sitter_docker::LANGUAGE.into(), (tree_sitter_docker::HIGHLIGHTS_QUERY, "", ""), "dockerfile")),
            "makefile" => return Some((tree_sitter_bash::LANGUAGE.into(), (tree_sitter_bash::HIGHLIGHT_QUERY, "", ""), "makefile")),
            "maidfile" => return Some((tree_sitter_toml_ng::language(), (tree_sitter_toml_ng::HIGHLIGHTS_QUERY, "", ""), "maidfile")),
            _ if file_name_str.starts_with('.') => {
                let without_dot = file_name_str.trim_start_matches('.');
                match without_dot {
                    "zshrc" | "bashrc" | "bash_profile" | "zprofile" | "gitignore" | "gitattributes" => {
                        return Some((tree_sitter_bash::LANGUAGE.into(), (tree_sitter_bash::HIGHLIGHT_QUERY, "", ""), "rc"))
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    match file_name.extension().and_then(|s| s.to_str()) {
        // Some("abap") => Some((tree_sitter_abap::LANGUAGE.into(), "abap")),
        // Some("ada") => Some((tree_sitter_ada::LANGUAGE.into(), "ada")),
        // Some("ahk" | "ahkl") => Some((tree_sitter_autohotkey::LANGUAGE.into(), "autohotkey")),
        // Some("applescript" | "scpt") => Some((tree_sitter_applescript::LANGUAGE.into(), "applescript")),
        // Some("arc") => Some((tree_sitter_arc::LANGUAGE.into(), "arc")),
        // Some("asp" | "asax" | "ascx" | "ashx" | "asmx" | "aspx" | "axd") => Some((tree_sitter_asp::LANGUAGE.into(), "asp")),
        // Some("as") => Some((tree_sitter_actionscript::LANGUAGE.into(), "actionscript")),
        // Some("asc" | "ash") => Some((tree_sitter_ags_script::LANGUAGE.into(), "ags_script")),
        // Some("awk" | "auk" | "gawk" | "mawk" | "nawk") => Some((tree_sitter_awk::LANGUAGE.into(), "awk")),
        // Some("bat" | "cmd") => Some((tree_sitter_batch::LANGUAGE.into(), "batch")),
        // Some("b" | "bf") => Some((tree_sitter_brainfuck::LANGUAGE.into(), "brainfuck")),
        // Some("cmake") => Some((tree_sitter_cmake::LANGUAGE.into(), "cmake")),
        // Some("cbl" | "cobol" | "cob") => Some((tree_sitter_cobol::LANGUAGE.into(), "cobol")),
        // Some("class" | "java") => Some((tree_sitter_java::LANGUAGE.into(), "java")),
        // Some("clj" | "cl2" | "cljs" | "cljx" | "cljc") => Some((tree_sitter_clojure::LANGUAGE.into(), "clojure")),
        // Some("coffee") => Some((tree_sitter_coffeescript::LANGUAGE.into(), "coffeescript")),
        // Some("cr") => Some((tree_sitter_crystal::LANGUAGE.into(), "crystal")),
        // Some("cu" | "cuh") => Some((tree_sitter_cuda::LANGUAGE.into(), "cuda")),
        // Some("cs" | "cshtml" | "csx") => Some((tree_sitter_c_sharp::LANGUAGE.into(), "c_sharp")),
        // Some("csv") => Some((tree_sitter_csv::LANGUAGE.into(), "csv")),
        // Some("d" | "di") => Some((tree_sitter_d::LANGUAGE.into(), "d")),
        // Some("dart") => Some((tree_sitter_dart::LANGUAGE.into(), "dart")),
        // Some("diff" | "patch") => Some((tree_sitter_diff::LANGUAGE.into(), "diff")),
        // Some("ex" | "exs") => Some((tree_sitter_elixir::LANGUAGE.into(), "elixir")),
        // Some("elm") => Some((tree_sitter_elm::LANGUAGE.into(), "elm")),
        // Some("el") => Some((tree_sitter_emacs_lisp::LANGUAGE.into(), "emacs_lisp")),
        // Some("erb") => Some((tree_sitter_erb::LANGUAGE.into(), "erb")),
        // Some("erl" | "es") => Some((tree_sitter_erlang::LANGUAGE.into(), "erlang")),
        // Some("fs" | "fsi" | "fsx") => Some((tree_sitter_f_sharp::LANGUAGE.into(), "f_sharp")),
        // Some("f" | "f90" | "fpp" | "for") => Some((tree_sitter_fortran::LANGUAGE.into(), "fortran")),
        // Some("fish") => Some((tree_sitter_fish::LANGUAGE.into(), "fish")),
        // Some("fth") => Some((tree_sitter_forth::LANGUAGE.into(), "forth")),
        // Some("g4") => Some((tree_sitter_antlr::LANGUAGE.into(), "antlr")),
        // Some("gd") => Some((tree_sitter_gdscript::LANGUAGE.into(), "gdscript")),
        // Some("glsl" | "vert" | "shader" | "geo" | "fshader" | "vrx" | "vsh" | "vshader" | "frag") => Some((tree_sitter_glsl::LANGUAGE.into(), "glsl")),
        // Some("gnu" | "gp" | "plot") => Some((tree_sitter_gnuplot::LANGUAGE.into(), "gnuplot")),
        // Some("groovy" | "gvy") => Some((tree_sitter_groovy::LANGUAGE.into(), "groovy")),
        // Some("hlsl") => Some((tree_sitter_hlsl::LANGUAGE.into(), "hlsl")),
        // Some("haml") => Some((tree_sitter_haml::LANGUAGE.into(), "haml")),
        // Some("handlebars" | "hbs") => Some((tree_sitter_handlebars::LANGUAGE.into(), "handlebars")),
        // Some("hs") => Some((tree_sitter_haskell::LANGUAGE.into(), "haskell")),
        // Some("ini" | "cfg") => Some((tree_sitter_ini::LANGUAGE.into(), "ini")),
        // Some("ino") => Some((tree_sitter_arduino::LANGUAGE.into(), "arduino")),
        // Some("ijs") => Some((tree_sitter_j::LANGUAGE.into(), "j")),
        // Some("jl") => Some((tree_sitter_julia::LANGUAGE.into(), "julia")),
        // Some("kt" | "ktm" | "kts") => Some((tree_sitter_kotlin::LANGUAGE.into(), "kotlin")),
        // Some("ll") => Some((tree_sitter_llvm::LANGUAGE.into(), "llvm")),
        // Some("l" | "lex") => Some((tree_sitter_lex::LANGUAGE.into(), "lex")),
        // Some("ls") => Some((tree_sitter_livescript::LANGUAGE.into(), "livescript")),
        // Some("lol") => Some((tree_sitter_lolcode::LANGUAGE.into(), "lolcode")),
        // Some("lisp" | "asd" | "lsp") => Some((tree_sitter_common_lisp::LANGUAGE.into(), "common_lisp")),
        // Some("m4") => Some((tree_sitter_m4::LANGUAGE.into(), "m4")),
        // Some("man" | "roff") => Some((tree_sitter_groff::LANGUAGE.into(), "groff")),
        // Some("matlab") => Some((tree_sitter_matlab::LANGUAGE.into(), "matlab")),
        // Some("m") => Some((tree_sitter_objective_c::LANGUAGE.into(), "objective_c")),
        // Some("ml") => Some((tree_sitter_ocaml::LANGUAGE.into(), "ocaml")),
        // Some("mk" | "mak") => Some((tree_sitter_makefile::LANGUAGE.into(), "makefile")),
        // Some("nix") => Some((tree_sitter_nix::LANGUAGE.into(), "nix")),
        // Some("numpy") => Some((tree_sitter_numpy::LANGUAGE.into(), "numpy")),
        // Some("opencl" | "cl") => Some((tree_sitter_opencl::LANGUAGE.into(), "opencl")),
        // Some("php") => Some((tree_sitter_php::LANGUAGE.into(), "php")),
        // Some("pas") => Some((tree_sitter_pascal::LANGUAGE.into(), "pascal")),
        // Some("pl") => Some((tree_sitter_perl::LANGUAGE.into(), "perl")),
        // Some("psl") => Some((tree_sitter_powershell::LANGUAGE.into(), "powershell")),
        // Some("pro") => Some((tree_sitter_prolog::LANGUAGE.into(), "prolog")),
        // Some("rst") => Some((tree_sitter_restructuredtext::LANGUAGE.into(), "restructuredtext")),
        // Some("rkt") => Some((tree_sitter_racket::LANGUAGE.into(), "racket")),
        // Some("rb" | "ruby") => Some((tree_sitter_ruby::LANGUAGE.into(), "ruby")),
        // Some("sql") => Some((tree_sitter_sql::LANGUAGE.into(), "sql")),
        // Some("sass") => Some((tree_sitter_sass::LANGUAGE.into(), "sass")),
        // Some("scala") => Some((tree_sitter_scala::LANGUAGE.into(), "scala")),
        // Some("scm") => Some((tree_sitter_scheme::LANGUAGE.into(), "scheme")),
        // Some("st") => Some((tree_sitter_smalltalk::LANGUAGE.into(), "smalltalk")),
        // Some("swift") => Some((tree_sitter_swift::LANGUAGE.into(), "swift")),
        // Some("tcl") => Some((tree_sitter_tcl::LANGUAGE.into(), "tcl")),
        // Some("vala") => Some((tree_sitter_vala::LANGUAGE.into(), "vala")),
        // Some("vb" | "vbs") => Some((tree_sitter_visual_basic::LANGUAGE.into(), "visual_basic")),
        // Some("vue") => Some((tree_sitter_vue::LANGUAGE.into(), "vue")),
        // Some("xm" | "x" | "xi") => Some((tree_sitter_logos::LANGUAGE.into(), "logos")),
        // Some("xml") => Some((tree_sitter_xml::LANGUAGE.into(), "xml")),
        // Some("y" | "yacc") => Some((tree_sitter_yacc::LANGUAGE.into(), "yacc")),
        // Some("yxx") => Some((tree_sitter_bison::LANGUAGE.into(), "bison")),
        Some("asm" | "nasm") => Some((tree_sitter_asm::LANGUAGE.into(), (tree_sitter_asm::HIGHLIGHTS_QUERY, tree_sitter_asm::INJECTIONS_QUERY, ""), "assembly")),
        Some("c") => Some((tree_sitter_c::LANGUAGE.into(), (tree_sitter_c::HIGHLIGHT_QUERY, "", ""), "c")),
        Some("cpp" | "cxx" | "cc") => Some((tree_sitter_c::LANGUAGE.into(), (tree_sitter_c::HIGHLIGHT_QUERY, "", ""), "cpp")),
        Some("css") => Some((tree_sitter_css::LANGUAGE.into(), (tree_sitter_css::HIGHLIGHTS_QUERY, "", ""), "css")),
        Some("go") => Some((tree_sitter_go::LANGUAGE.into(), (tree_sitter_go::HIGHLIGHTS_QUERY, "", ""), "go")),
        Some("h") => Some((tree_sitter_c::LANGUAGE.into(), (tree_sitter_c::HIGHLIGHT_QUERY, "", ""), "c_header")),
        Some("hpp") => Some((tree_sitter_c::LANGUAGE.into(), (tree_sitter_c::HIGHLIGHT_QUERY, "", ""), "cpp_header")),
        Some("html" | "htm" | "xhtml") => Some((tree_sitter_html::LANGUAGE.into(), (tree_sitter_html::HIGHLIGHTS_QUERY, tree_sitter_html::INJECTIONS_QUERY, ""), "html")),
        Some("json") => Some((tree_sitter_json::LANGUAGE.into(), (tree_sitter_json::HIGHLIGHTS_QUERY, "", ""), "json")),
        Some("lua") => Some((tree_sitter_lua::LANGUAGE.into(), (tree_sitter_lua::HIGHLIGHTS_QUERY, tree_sitter_lua::INJECTIONS_QUERY, ""), "lua")),
        Some("md" | "markdown") => Some((tree_sitter_md::LANGUAGE.into(), (tree_sitter_md::HIGHLIGHT_QUERY_BLOCK, "", ""), "markdown")),
        Some("py" | "pyw") => Some((tree_sitter_python::LANGUAGE.into(), (tree_sitter_python::HIGHLIGHTS_QUERY, "", ""), "python")),
        Some("r") => Some((tree_sitter_r::LANGUAGE.into(), (tree_sitter_r::HIGHLIGHTS_QUERY, "", ""), "r")),
        Some("rs") => Some((tree_sitter_rust::LANGUAGE.into(), (tree_sitter_rust::HIGHLIGHTS_QUERY, "", ""), "rust")),
        Some("sh") => Some((tree_sitter_bash::LANGUAGE.into(), (tree_sitter_bash::HIGHLIGHT_QUERY, "", ""), "bash")),
        Some("scss") => Some((tree_sitter_css::LANGUAGE.into(), (tree_sitter_css::HIGHLIGHTS_QUERY, "", ""), "scss")),
        Some("toml") => Some((tree_sitter_toml_ng::language(), (tree_sitter_toml_ng::HIGHLIGHTS_QUERY, "", ""), "toml")),
        Some("yaml" | "yml") => Some((tree_sitter_yaml::language(), (tree_sitter_yaml::HIGHLIGHTS_QUERY, "", ""), "yaml")),
        Some("zsh") => Some((tree_sitter_bash::LANGUAGE.into(), (tree_sitter_bash::HIGHLIGHT_QUERY, "", ""), "zsh")),

        Some("js") => Some((
            tree_sitter_javascript::LANGUAGE.into(),
            (tree_sitter_javascript::HIGHLIGHT_QUERY, tree_sitter_javascript::INJECTIONS_QUERY, ""),
            "javascript",
        )),

        Some("ts") => Some((
            tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
            (concatcp!(tree_sitter_typescript::HIGHLIGHTS_QUERY, tree_sitter_javascript::HIGHLIGHT_QUERY), "", ""),
            "typescript",
        )),

        Some("jsx") => Some((
            tree_sitter_javascript::LANGUAGE.into(),
            (
                concatcp!(tree_sitter_javascript::JSX_HIGHLIGHT_QUERY, tree_sitter_javascript::HIGHLIGHT_QUERY),
                tree_sitter_javascript::INJECTIONS_QUERY,
                "",
            ),
            "jsx",
        )),

        Some("tsx") => Some((
            tree_sitter_typescript::LANGUAGE_TSX.into(),
            (concatcp!(tree_sitter_typescript::HIGHLIGHTS_QUERY, tree_sitter_javascript::JSX_HIGHLIGHT_QUERY), "", ""),
            "tsx",
        )),

        _ => None,
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
