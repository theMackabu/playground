pub type Config = (tree_sitter::Language, (&'static str, &'static str, &'static str), &'static str);

pub mod ada {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_ada() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_ada) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/ada/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = include_str!("../languages/ada/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "ada") }
}

pub mod asm {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_asm() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_asm) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/asm/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/asm/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "assembly") }
}

pub mod awk {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_awk() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_awk) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/awk/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/awk/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "awk") }
}

pub mod bash {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_bash() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_bash) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/bash/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "bash") }
}

pub mod bibtex {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_bibtex() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_bibtex) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/bibtex/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = include_str!("../languages/bibtex/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "bibtex") }
}

pub mod bicep {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_bicep() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_bicep) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/bicep/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/bicep/queries/injections.scm");
    pub const LOCALS_QUERY: &str = include_str!("../languages/bicep/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "bicep") }
}

pub mod blueprint {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_blueprint() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_blueprint) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/blueprint/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "blueprint") }
}

pub mod c {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_c() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_c) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/c/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/c/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "c") }
}

pub mod capnp {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_capnp() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_capnp) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/capnp/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/capnp/queries/injections.scm");
    pub const LOCALS_QUERY: &str = include_str!("../languages/capnp/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "capnp") }
}

pub mod clojure {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_clojure() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_clojure) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/clojure/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/clojure/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "clojure") }
}

pub mod c_sharp {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_c_sharp() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_c_sharp) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/c_sharp/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "c_sharp") }
}

pub mod cpp {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_cpp() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_cpp) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/cpp/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/cpp/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "cpp") }
}

pub mod css {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_css() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_css) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/css/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/css/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "css") }
}

pub mod cue {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_cue() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_cue) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/cue/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/cue/queries/injections.scm");
    pub const LOCALS_QUERY: &str = include_str!("../languages/cue/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "cue") }
}

pub mod d {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_d() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_d) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/d/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/d/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "d") }
}

pub mod dart {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_dart() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_dart) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/dart/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "dart") }
}

pub mod diff {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_diff() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_diff) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/diff/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "diff") }
}

pub mod dockerfile {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_dockerfile() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_dockerfile) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/dockerfile/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/dockerfile/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "dockerfile") }
}

pub mod eex {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_eex() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_eex) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/eex/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/eex/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "eex") }
}

pub mod elisp {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_elisp() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_elisp) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/elisp/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "elisp") }
}

pub mod elixir {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_elixir() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_elixir) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/elixir/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/elixir/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "elixir") }
}

pub mod elm {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_elm() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_elm) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/elm/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/elm/queries/injections.scm");
    pub const LOCALS_QUERY: &str = include_str!("../languages/elm/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "elm") }
}

pub mod erlang {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_erlang() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_erlang) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/erlang/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "erlang") }
}

pub mod forth {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_forth() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_forth) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/forth/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "forth") }
}

pub mod fortran {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_fortran() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_fortran) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/fortran/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "fortran") }
}

pub mod fish {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_fish() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_fish) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/fish/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "fish") }
}

pub mod gdscript {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_gdscript() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_gdscript) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/gdscript/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/gdscript/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "gdscript") }
}

pub mod gleam {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_gleam() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_gleam) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/gleam/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/gleam/queries/injections.scm");
    pub const LOCALS_QUERY: &str = include_str!("../languages/gleam/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "gleam") }
}

pub mod glsl {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_glsl() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_glsl) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/glsl/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/glsl/queries/injections.scm");
    pub const LOCALS_QUERY: &str = include_str!("../languages/glsl/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "glsl") }
}

pub mod go {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_go() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_go) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/go/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "go") }
}

pub mod haskell {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_haskell() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_haskell) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/haskell/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/haskell/queries/injections.scm");
    pub const LOCALS_QUERY: &str = include_str!("../languages/haskell/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "haskell") }
}

pub mod hcl {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_hcl() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_hcl) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/hcl/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/hcl/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "hcl") }
}

pub mod heex {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_heex() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_heex) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/heex/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/heex/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "heex") }
}

pub mod html {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_html() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_html) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/html/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/html/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "html") }
}

pub mod ini {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_ini() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_ini) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/ini/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "ini") }
}

pub mod java {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_java() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_java) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/java/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "java") }
}

pub mod javascript {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_javascript() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_javascript) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/javascript/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/javascript/queries/injections.scm");
    pub const LOCALS_QUERY: &str = include_str!("../languages/javascript/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "javascript") }
}

pub mod json {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_json() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_json) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/json/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "json") }
}

pub mod jsx {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_jsx() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_jsx) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/jsx/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/jsx/queries/injections.scm");
    pub const LOCALS_QUERY: &str = include_str!("../languages/jsx/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "jsx") }
}

pub mod julia {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_julia() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_julia) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/julia/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "julia") }
}

pub mod kotlin {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_kotlin() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_kotlin) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/kotlin/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "kotlin") }
}

pub mod latex {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_latex() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_latex) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/latex/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/latex/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "latex") }
}

pub mod llvm {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_llvm() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_llvm) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/llvm/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/llvm/queries/injections.scm");
    pub const LOCALS_QUERY: &str = include_str!("../languages/llvm/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "llvm") }
}

pub mod lua {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_lua() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_lua) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/lua/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/lua/queries/injections.scm");
    pub const LOCALS_QUERY: &str = include_str!("../languages/lua/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "lua") }
}

pub mod make {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_make() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_make) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/make/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/make/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "make") }
}

pub mod markdown {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_markdown() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_markdown) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/markdown/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "markdown") }
}

pub mod matlab {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_matlab() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_matlab) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/matlab/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/matlab/queries/injections.scm");
    pub const LOCALS_QUERY: &str = include_str!("../languages/matlab/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "matlab") }
}

pub mod meson {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_meson() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_meson) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/meson/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "meson") }
}

pub mod nix {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_nix() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_nix) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/nix/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/nix/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "nix") }
}

pub mod objc {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_objc() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_objc) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/objc/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/objc/queries/injections.scm");
    pub const LOCALS_QUERY: &str = include_str!("../languages/objc/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "objc") }
}

pub mod ocaml {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_ocaml() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_ocaml) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/ocaml/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = include_str!("../languages/ocaml/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "ocaml") }
}

pub mod ocaml_interface {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_ocaml_interface() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_ocaml_interface) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/ocaml_interface/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/ocaml_interface/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "ocaml_interface") }
}

pub mod openscad {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_openscad() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_openscad) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/openscad/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/openscad/queries/injections.scm");
    pub const LOCALS_QUERY: &str = include_str!("../languages/openscad/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "openscad") }
}

pub mod pascal {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_pascal() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_pascal) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/pascal/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/pascal/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "pascal") }
}

pub mod php {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_php() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_php) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/php/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/php/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "php") }
}

pub mod plaintext {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_plaintext() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_plaintext) };
    pub const HIGHLIGHT_QUERY: &str = "";
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "plaintext") }
}

pub mod proto {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_proto() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_proto) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/proto/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "proto") }
}

pub mod python {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_python() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_python) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/python/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/python/queries/injections.scm");
    pub const LOCALS_QUERY: &str = include_str!("../languages/python/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "python") }
}

pub mod r {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_r() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_r) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/r/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = include_str!("../languages/r/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "r") }
}

pub mod racket {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_racket() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_racket) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/racket/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = include_str!("../languages/racket/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "racket") }
}

pub mod regex {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_regex() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_regex) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/regex/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "regex") }
}

pub mod ruby {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_ruby() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_ruby) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/ruby/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/ruby/queries/injections.scm");
    pub const LOCALS_QUERY: &str = include_str!("../languages/ruby/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "ruby") }
}

pub mod rust {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_rust() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_rust) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/rust/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/rust/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "rust") }
}

pub mod scala {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_scala() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_scala) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/scala/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/scala/queries/injections.scm");
    pub const LOCALS_QUERY: &str = include_str!("../languages/scala/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "scala") }
}

pub mod scheme {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_scheme() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_scheme) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/scheme/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "scheme") }
}

pub mod scss {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_scss() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_scss) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/scss/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/scss/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "scss") }
}

pub mod sql {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_sql() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_sql) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/sql/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "sql") }
}

pub mod svelte {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_svelte() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_svelte) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/svelte/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/svelte/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "svelte") }
}

pub mod swift {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_swift() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_swift) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/swift/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = include_str!("../languages/swift/queries/locals.scm");

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "swift") }
}

pub mod toml {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_toml() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_toml) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/toml/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/toml/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "toml") }
}

pub mod typescript {
    use super::Config;
    use const_format::concatcp;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_typescript() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_typescript) };
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = include_str!("../languages/typescript/queries/locals.scm");

    // combine for syntax
    pub const HIGHLIGHT_QUERY: &str = concatcp!(
        include_str!("../languages/typescript/queries/highlights.scm"),
        include_str!("../languages/javascript/queries/highlights.scm"),
    );

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "typescript") }
}

pub mod tsx {
    use super::Config;
    use const_format::concatcp;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_tsx() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_tsx) };
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = include_str!("../languages/tsx/queries/locals.scm");

    // combine for syntax
    pub const HIGHLIGHT_QUERY: &str = concatcp!(include_str!("../languages/tsx/queries/highlights.scm"), include_str!("../languages/jsx/queries/highlights.scm"),);

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "tsx") }
}

pub mod vim {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_vim() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_vim) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/vim/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/vim/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "vim") }
}

pub mod wast {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_wast() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_wast) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/wast/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "wast") }
}

pub mod wat {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_wat() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_wat) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/wat/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "wat") }
}

pub mod x86asm {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_x86asm() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_x86asm) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/x86asm/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "x86asm") }
}

pub mod wgsl {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_wgsl() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_wgsl) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/wgsl/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = "";
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "wgsl") }
}

pub mod yaml {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_yaml() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_yaml) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/yaml/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/yaml/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "yaml") }
}

pub mod zig {
    use super::Config;
    use tree_sitter_language::LanguageFn;

    extern "C" {
        pub fn tree_sitter_zig() -> *const ();
    }

    pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_zig) };
    pub const HIGHLIGHT_QUERY: &str = include_str!("../languages/zig/queries/highlights.scm");
    pub const INJECTIONS_QUERY: &str = include_str!("../languages/zig/queries/injections.scm");
    pub const LOCALS_QUERY: &str = "";

    pub fn config() -> Config { (LANGUAGE.into(), (HIGHLIGHT_QUERY, INJECTIONS_QUERY, LOCALS_QUERY), "zig") }
}

#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    Runtime(fn() -> Config),
    Plaintext,
    Ada,
    Asm,
    Awk,
    Bash,
    Bibtex,
    Bicep,
    Blueprint,
    C,
    Capnp,
    Clojure,
    CSharp,
    Cpp,
    Css,
    Cue,
    D,
    Dart,
    Diff,
    Dockerfile,
    Eex,
    Elisp,
    Elixir,
    Elm,
    Erlang,
    Forth,
    Fortran,
    Fish,
    Gdscript,
    Gleam,
    Glsl,
    Go,
    Haskell,
    Hcl,
    Heex,
    Html,
    Ini,
    Java,
    Javascript,
    Json,
    Jsx,
    Julia,
    Kotlin,
    Latex,
    Llvm,
    Lua,
    Make,
    Markdown,
    Matlab,
    Meson,
    Nix,
    ObjectiveC,
    Ocaml,
    OcamlInterface,
    OpenScad,
    Pascal,
    Php,
    ProtoBuf,
    Python,
    R,
    Racket,
    Regex,
    Ruby,
    Rust,
    Scala,
    Scheme,
    Scss,
    Sql,
    Svelte,
    Swift,
    Toml,
    Typescript,
    Tsx,
    Vimscript,
    Wast,
    Wat,
    X86asm,
    Wgsl,
    Yaml,
    Zig,
}

impl Language {
    pub fn from_token(token: impl AsRef<str>) -> Option<Self> {
        match token.as_ref() {
            "ada" => Some(Self::Ada),
            "asm" | "assembly" | "assembler" | "nasm" => Some(Self::Asm),
            "awk" => Some(Self::Awk),
            "bash" => Some(Self::Bash),
            "sh" => Some(Self::Bash),
            "shell" => Some(Self::Bash),
            "bibtex" => Some(Self::Bibtex),
            "bib" => Some(Self::Bibtex),
            "bicep" => Some(Self::Bicep),
            "blueprint" => Some(Self::Blueprint),
            "blp" => Some(Self::Blueprint),
            "c" => Some(Self::C),
            "h" => Some(Self::C),
            "capnp" => Some(Self::Capnp),
            "clojure" => Some(Self::Clojure),
            "clj" => Some(Self::Clojure),
            "cljc" => Some(Self::Clojure),
            "c_sharp" => Some(Self::CSharp),
            "c#" => Some(Self::CSharp),
            "csharp" => Some(Self::CSharp),
            "cs" => Some(Self::CSharp),
            "cpp" => Some(Self::Cpp),
            "c++" => Some(Self::Cpp),
            "hpp" => Some(Self::Cpp),
            "h++" => Some(Self::Cpp),
            "cc" => Some(Self::Cpp),
            "hh" => Some(Self::Cpp),
            "css" => Some(Self::Css),
            "cue" => Some(Self::Cue),
            "d" => Some(Self::D),
            "dlang" => Some(Self::D),
            "dart" => Some(Self::Dart),
            "diff" => Some(Self::Diff),
            "dockerfile" => Some(Self::Dockerfile),
            "docker" => Some(Self::Dockerfile),
            "eex" => Some(Self::Eex),
            "elisp" => Some(Self::Elisp),
            "el" => Some(Self::Elisp),
            "emacs-lisp" => Some(Self::Elisp),
            "elixir" => Some(Self::Elixir),
            "ex" => Some(Self::Elixir),
            "exs" => Some(Self::Elixir),
            "leex" => Some(Self::Elixir),
            "elm" => Some(Self::Elm),
            "erlang" => Some(Self::Erlang),
            "erl" => Some(Self::Erlang),
            "hrl" => Some(Self::Erlang),
            "es" => Some(Self::Erlang),
            "escript" => Some(Self::Erlang),
            "forth" => Some(Self::Forth),
            "fth" => Some(Self::Forth),
            "fortran" => Some(Self::Fortran),
            "for" => Some(Self::Fortran),
            "fish" => Some(Self::Fish),
            "gdscript" => Some(Self::Gdscript),
            "gd" => Some(Self::Gdscript),
            "gleam" => Some(Self::Gleam),
            "glsl" => Some(Self::Glsl),
            "go" => Some(Self::Go),
            "golang" => Some(Self::Go),
            "haskell" => Some(Self::Haskell),
            "hs" => Some(Self::Haskell),
            "hcl" => Some(Self::Hcl),
            "terraform" => Some(Self::Hcl),
            "heex" => Some(Self::Heex),
            "html" => Some(Self::Html),
            "htm" => Some(Self::Html),
            "ini" => Some(Self::Ini),
            "java" => Some(Self::Java),
            "javascript" => Some(Self::Javascript),
            "js" => Some(Self::Javascript),
            "json" => Some(Self::Json),
            "jsx" => Some(Self::Jsx),
            "julia" => Some(Self::Julia),
            "jl" => Some(Self::Julia),
            "kotlin" => Some(Self::Kotlin),
            "kt" => Some(Self::Kotlin),
            "kts" => Some(Self::Kotlin),
            "latex" => Some(Self::Latex),
            "tex" => Some(Self::Latex),
            "llvm" => Some(Self::Llvm),
            "lua" => Some(Self::Lua),
            "md" => Some(Self::Markdown),
            "make" | "mk" | "makefile" => Some(Self::Make),
            "matlab" => Some(Self::Matlab),
            "m" => Some(Self::Matlab),
            "meson" => Some(Self::Meson),
            "nix" => Some(Self::Nix),
            "objc" => Some(Self::ObjectiveC),
            "objective_c" => Some(Self::ObjectiveC),
            "ocaml" => Some(Self::Ocaml),
            "ml" => Some(Self::Ocaml),
            "ocaml_interface" => Some(Self::OcamlInterface),
            "mli" => Some(Self::OcamlInterface),
            "openscad" => Some(Self::OpenScad),
            "scad" => Some(Self::OpenScad),
            "pascal" => Some(Self::Pascal),
            "php" => Some(Self::Php),
            "plaintext" => Some(Self::Plaintext),
            "none" => Some(Self::Plaintext),
            "nolang" => Some(Self::Plaintext),
            "proto" => Some(Self::ProtoBuf),
            "protobuf" => Some(Self::ProtoBuf),
            "python" => Some(Self::Python),
            "py" => Some(Self::Python),
            "r" => Some(Self::R),
            "racket" => Some(Self::Racket),
            "rkt" => Some(Self::Racket),
            "regex" => Some(Self::Regex),
            "ruby" => Some(Self::Ruby),
            "rb" => Some(Self::Ruby),
            "rust" => Some(Self::Rust),
            "rs" => Some(Self::Rust),
            "scala" => Some(Self::Scala),
            "scheme" => Some(Self::Scheme),
            "scm" => Some(Self::Scheme),
            "ss" => Some(Self::Scheme),
            "scss" | "saas" => Some(Self::Scss),
            "sql" => Some(Self::Sql),
            "svelte" => Some(Self::Svelte),
            "swift" => Some(Self::Swift),
            "toml" => Some(Self::Toml),
            "typescript" => Some(Self::Typescript),
            "ts" => Some(Self::Typescript),
            "tsx" => Some(Self::Tsx),
            "vim" => Some(Self::Vimscript),
            "vimscript" => Some(Self::Vimscript),
            "wast" => Some(Self::Wast),
            "wat" => Some(Self::Wat),
            "wasm" => Some(Self::Wat),
            "x86asm" => Some(Self::X86asm),
            "x86" => Some(Self::X86asm),
            "wgsl" => Some(Self::Wgsl),
            "yaml" => Some(Self::Yaml),
            "zig" => Some(Self::Zig),
            _ => None,
        }
    }

    pub fn config(&self) -> Config {
        match *self {
            Self::Ada => ada::config(),
            Self::Asm => asm::config(),
            Self::Awk => awk::config(),
            Self::Bash => bash::config(),
            Self::Bibtex => bibtex::config(),
            Self::Bicep => bicep::config(),
            Self::Blueprint => blueprint::config(),
            Self::C => c::config(),
            Self::Capnp => capnp::config(),
            Self::Clojure => clojure::config(),
            Self::CSharp => c_sharp::config(),
            Self::Cpp => cpp::config(),
            Self::Css => css::config(),
            Self::Cue => cue::config(),
            Self::D => d::config(),
            Self::Dart => dart::config(),
            Self::Diff => diff::config(),
            Self::Dockerfile => dockerfile::config(),
            Self::Eex => eex::config(),
            Self::Elisp => elisp::config(),
            Self::Elixir => elixir::config(),
            Self::Elm => elm::config(),
            Self::Erlang => erlang::config(),
            Self::Forth => forth::config(),
            Self::Fortran => fortran::config(),
            Self::Fish => fish::config(),
            Self::Gdscript => gdscript::config(),
            Self::Gleam => gleam::config(),
            Self::Glsl => glsl::config(),
            Self::Go => go::config(),
            Self::Haskell => haskell::config(),
            Self::Hcl => hcl::config(),
            Self::Heex => heex::config(),
            Self::Html => html::config(),
            Self::Ini => ini::config(),
            Self::Java => java::config(),
            Self::Javascript => javascript::config(),
            Self::Json => json::config(),
            Self::Jsx => jsx::config(),
            Self::Julia => julia::config(),
            Self::Kotlin => kotlin::config(),
            Self::Latex => latex::config(),
            Self::Llvm => llvm::config(),
            Self::Lua => lua::config(),
            Self::Make => make::config(),
            Self::Markdown => markdown::config(),
            Self::Matlab => matlab::config(),
            Self::Meson => meson::config(),
            Self::Nix => nix::config(),
            Self::ObjectiveC => objc::config(),
            Self::Ocaml => ocaml::config(),
            Self::OcamlInterface => ocaml_interface::config(),
            Self::OpenScad => openscad::config(),
            Self::Pascal => pascal::config(),
            Self::Php => php::config(),
            Self::Plaintext => plaintext::config(),
            Self::ProtoBuf => proto::config(),
            Self::Python => python::config(),
            Self::R => r::config(),
            Self::Racket => racket::config(),
            Self::Regex => regex::config(),
            Self::Ruby => ruby::config(),
            Self::Rust => rust::config(),
            Self::Scala => scala::config(),
            Self::Scheme => scheme::config(),
            Self::Scss => scss::config(),
            Self::Sql => sql::config(),
            Self::Svelte => svelte::config(),
            Self::Swift => swift::config(),
            Self::Toml => toml::config(),
            Self::Typescript => typescript::config(),
            Self::Tsx => tsx::config(),
            Self::Vimscript => vim::config(),
            Self::Wast => wast::config(),
            Self::Wat => wat::config(),
            Self::X86asm => x86asm::config(),
            Self::Wgsl => wgsl::config(),
            Self::Yaml => yaml::config(),
            Self::Zig => zig::config(),
            Self::Runtime(ptr) => ptr(),
        }
    }
}
