use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_asm() -> *const ();
}

pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_asm) };
pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/highlights.scm");
pub const INJECTIONS_QUERY: &'static str = include_str!("../../queries/injections.scm");
