use rustler::*;

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

impl From<Range> for tree_sitter::Range {
    fn from(r: Range) -> Self {
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

impl From<Point> for tree_sitter::Point {
    fn from(p: Point) -> Self {
        Self {
            row: p.row,
            column: p.column,
        }
    }
}

#[derive(NifStruct)]
#[module = "TreeSitter.InputEdit"]
pub struct InputEdit {
    pub start_byte: usize,
    pub old_end_byte: usize,
    pub new_end_byte: usize,
    pub start_position: Point,
    pub old_end_position: Point,
    pub new_end_position: Point,
}

impl From<tree_sitter::InputEdit> for InputEdit {
    fn from(i: tree_sitter::InputEdit) -> Self {
        Self {
            start_byte: i.start_byte,
            old_end_byte: i.old_end_byte,
            new_end_byte: i.new_end_byte,
            start_position: i.start_position.into(),
            old_end_position: i.old_end_position.into(),
            new_end_position: i.new_end_position.into(),
        }
    }
}

impl From<InputEdit> for tree_sitter::InputEdit {
    fn from(i: InputEdit) -> Self {
        Self {
            start_byte: i.start_byte,
            old_end_byte: i.old_end_byte,
            new_end_byte: i.new_end_byte,
            start_position: i.start_position.into(),
            old_end_position: i.old_end_position.into(),
            new_end_position: i.new_end_position.into(),
        }
    }
}
