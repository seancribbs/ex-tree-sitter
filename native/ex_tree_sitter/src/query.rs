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

#[derive(NifStruct)]
#[module = "TreeSitter.Node"]
pub struct Node {
    pub id: usize,
    pub text: Option<String>,
    pub range: self::Range,
    pub kind: String,
    pub kind_id: u16,
    pub is_named: bool,
    pub is_extra: bool,
    pub has_changes: bool,
    pub has_error: bool,
    pub is_error: bool,
    pub is_missing: bool,
    pub child_count: usize,
}

impl Node {
    pub fn from_tsnode(node: &tree_sitter::Node<'_>, source: Option<&[u8]>) -> Self {
        let text = source
            .and_then(|s| node.utf8_text(s).ok())
            .map(|s| s.to_string());

        Self {
            id: node.id(),
            text,
            range: node.range().into(),
            kind: node.kind().to_string(),
            kind_id: node.kind_id(),
            is_named: node.is_named(),
            is_extra: node.is_extra(),
            has_changes: node.has_changes(),
            has_error: node.has_error(),
            is_error: node.is_error(),
            is_missing: node.is_missing(),
            child_count: node.child_count(),
        }
    }
}

#[derive(NifStruct)]
#[module = "TreeSitter.Range"]
pub struct Range {
    pub start_byte: usize,
    pub end_byte: usize,
    pub start_point: Point,
    pub end_point: Point,
}

impl From<tree_sitter::Range> for Range {
    fn from(r: tree_sitter::Range) -> Self {
        Self {
            start_byte: r.start_byte,
            end_byte: r.end_byte,
            start_point: r.start_point.into(),
            end_point: r.end_point.into(),
        }
    }
}

#[derive(NifStruct)]
#[module = "TreeSitter.Point"]
pub struct Point {
    pub row: usize,
    pub column: usize,
}

impl From<tree_sitter::Point> for Point {
    fn from(p: tree_sitter::Point) -> Self {
        Self {
            row: p.row,
            column: p.column,
        }
    }
}
