use crate::document::*;
use crate::error::*;
use rustler::*;
use tree_sitter::{Query, QueryCursor};

pub fn query_matches(
    tree: &tree_sitter::Tree,
    language: tree_sitter::Language,
    query_raw: &[u8],
    source: &[u8],
) -> NifResult<Vec<QueryMatch>> {
    let query_source = String::from_utf8(query_raw.to_vec()).with_nif_error()?;
    let query = Query::new(language, &query_source).with_nif_error()?;
    let mut cursor = QueryCursor::new();
    Ok(cursor
        .matches(&query, tree.root_node(), source)
        .map(|m| QueryMatch::from_tsmatch(m, &query, source))
        .collect())
}

#[derive(NifStruct)]
#[module = "TreeSitter.QueryMatch"]
pub struct QueryMatch {
    pub pattern_index: usize,
    pub captures: Vec<QueryCapture>,
}

impl QueryMatch {
    pub fn from_tsmatch(
        tsmatch: tree_sitter::QueryMatch<'_, '_>,
        query: &tree_sitter::Query,
        source: &[u8],
    ) -> Self {
        let source = source.as_ref();
        let captures = tsmatch
            .captures
            .iter()
            .map(|c| QueryCapture::from_tscapture(c, query, source))
            .collect();
        Self {
            pattern_index: tsmatch.pattern_index,
            captures,
        }
    }
}

#[derive(NifStruct)]
#[module = "TreeSitter.QueryCapture"]
pub struct QueryCapture {
    pub node: Node,
    pub index: u32,
    pub capture_name: String,
}

impl QueryCapture {
    pub fn from_tscapture(
        capture: &tree_sitter::QueryCapture<'_>,
        query: &tree_sitter::Query,
        source: &[u8],
    ) -> Self {
        let capture_names = query.capture_names();
        let capture_name = capture_names[capture.index as usize].clone();
        let node = Node::from_tsnode(&capture.node, Some(source));
        Self {
            node,
            capture_name,
            index: capture.index,
        }
    }
}
