use rustler::*;

mod atoms;
mod error;
mod language;
mod parser;

use error::*;

//
// ---- API functions ----
//

#[nif]
pub fn language_supported(lang: language::Language) -> bool {
    lang.supported()
}

#[nif]
pub fn parser_new(lang: language::Language) -> NifResult<ResourceArc<parser::Parser>> {
    let lang_impl = lang
        .get_language()
        .ok_or(atoms::unsupported_language())
        .with_nif_error()?;
    parser::Parser::new(lang_impl)
        .with_nif_error()
        .map(ResourceArc::new)
}

#[nif(schedule = "DirtyCpu")]
pub fn parser_parse(
    parser: ResourceArc<parser::Parser>,
    text: Binary,
) -> Option<ResourceArc<parser::Tree>> {
    parser.parse(text.as_slice()).map(ResourceArc::new)
}

// #[nif]
// pub fn tree_root_node(tree: ResourceArc<parser::Tree>) -> ResourceArc<parser::Node> {

// }

//
// ---- NIF boilerplate ----
//

fn load(env: Env, _term: Term) -> bool {
    // Let tree-sitter use BEAM's allocator
    unsafe {
        tree_sitter::set_allocator(
            Some(rustler_sys::enif_alloc),
            None,
            Some(rustler_sys::enif_realloc),
            Some(rustler_sys::enif_free),
        );
    }
    parser::load(env)
}

rustler::init!(
    "Elixir.TreeSitter.NIF",
    [language_supported, parser_new, parser_parse],
    load = load
);
