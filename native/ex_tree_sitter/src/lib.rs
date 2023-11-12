use rustler::*;
use std::ops::Deref;

mod atoms;
mod error;
mod language;
mod parser;
mod query;

use error::*;

//
// ---- API functions ----
//

#[nif]
pub fn language_supported(lang: language::Language) -> bool {
    lang.supported()
}

#[nif]
pub fn language_queries(
    env: Env<'_>,
    lang: language::Language,
) -> std::collections::HashMap<Term<'_>, &'static str> {
    lang.queries()
        .into_iter()
        .map(|(k, v)| (k.to_term(env.clone()), v))
        .collect()
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

#[nif(schedule = "DirtyCpu")]
pub fn tree_root_node(tree: ResourceArc<parser::Tree>) -> query::Node {
    tree.root_node()
}

#[nif(schedule = "DirtyCpu")]
pub fn tree_pre_walk(tree: ResourceArc<parser::Tree>) -> Vec<query::Node> {
    tree.pre_walk()
}

#[nif(schedule = "DirtyCpu")]
pub fn query_matches(
    tree: ResourceArc<parser::Tree>,
    lang: language::Language,
    query_raw: Binary,
    source: Binary,
) -> NifResult<Vec<query::QueryMatch>> {
    let lang_impl = lang
        .get_language()
        .ok_or(atoms::unsupported_language())
        .with_nif_error()?;
    query::query_matches(
        tree.lock().with_nif_error()?.deref(),
        lang_impl,
        query_raw.as_slice(),
        source.as_slice(),
    )
}

//
// ---- NIF boilerplate ----
//

fn load(env: Env, _term: Term) -> bool {
    // TODO: Let tree-sitter use BEAM's allocator.
    // I think this might have been a source of segfaults on query execution??
    // unsafe {
    //     tree_sitter::set_allocator(
    //         Some(rustler_sys::enif_alloc),
    //         None,
    //         Some(rustler_sys::enif_realloc),
    //         Some(rustler_sys::enif_free),
    //     );
    // }
    parser::load(env)
}

rustler::init!(
    "Elixir.TreeSitter.NIF",
    [
        language_supported,
        language_queries,
        parser_new,
        parser_parse,
        tree_root_node,
        tree_pre_walk,
        query_matches
    ],
    load = load
);
