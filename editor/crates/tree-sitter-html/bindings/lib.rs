use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_html() -> *const ();
}

pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_html) };
pub const NODE_TYPES: &str = include_str!("../src/node-types.json");
pub const HIGHLIGHTS_QUERY: &str = include_str!("../queries/highlights.scm");
pub const INJECTIONS_QUERY: &str = include_str!("../queries/injections.scm");
