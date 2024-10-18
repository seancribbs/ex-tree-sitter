use rustler::*;
use std::ops::Deref;

mod atoms;
mod document;
mod error;
mod language;
mod parser;
mod query;

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
    let lang_impl = lang.get_language().ok_or(unsupported_language_error())?;
    let parser = parser::Parser::new(lang_impl)?;
    Ok(ResourceArc::new(parser))
}

#[nif]
pub fn parser_set_language(
    parser: ResourceArc<parser::Parser>,
    lang: language::Language,
) -> NifResult<()> {
    let lang_impl = lang.get_language().ok_or(unsupported_language_error())?;
    Ok(parser.set_language(lang_impl)?)
}

#[nif]
pub fn parser_set_included_ranges(
    parser: ResourceArc<parser::Parser>,
    ranges: Vec<document::Range>,
) -> NifResult<()> {
    let ranges: Vec<tree_sitter::Range> = ranges.into_iter().map(Into::into).collect();
    Ok(parser.set_included_ranges(&ranges)?)
}

#[nif(schedule = "DirtyCpu")]
pub fn parser_parse(
    parser: ResourceArc<parser::Parser>,
    text: Binary,
    old_tree: Option<ResourceArc<parser::Tree>>,
) -> Option<ResourceArc<parser::Tree>> {
    parser
        .parse(text.as_slice(), old_tree.as_deref())
        .map(ResourceArc::new)
}

#[nif(schedule = "DirtyCpu")]
pub fn tree_edit(tree: ResourceArc<parser::Tree>, edit: document::InputEdit) {
    tree.edit(edit);
}

#[nif(schedule = "DirtyCpu")]
pub fn tree_root_node(tree: ResourceArc<parser::Tree>) -> document::Node {
    tree.root_node()
}

#[nif(schedule = "DirtyCpu")]
pub fn tree_pre_walk(tree: ResourceArc<parser::Tree>) -> Vec<document::Node> {
    tree.pre_walk()
}

#[nif(schedule = "DirtyCpu")]
pub fn query_matches(
    tree: ResourceArc<parser::Tree>,
    lang: language::Language,
    query_raw: Binary,
    source: Binary,
) -> NifResult<Vec<query::QueryMatch>> {
    let lang_impl = lang.get_language().ok_or(unsupported_language_error())?;
    Ok(query::query_matches(
        tree.lock()?.deref(),
        lang_impl,
        query_raw.as_slice(),
        source.as_slice(),
    )?)
}

//
// ---- NIF boilerplate ----
//

fn unsupported_language_error() -> rustler::error::Error {
    rustler::error::Error::Term(Box::new(atoms::unsupported_language()))
}

rustler::init!(
    "Elixir.TreeSitter.NIF"
);
