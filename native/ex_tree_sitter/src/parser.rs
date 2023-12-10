use crate::document::*;
use crate::error::Error;
use rustler::{resource, Env};
use std::sync::{Mutex, MutexGuard};

pub type ParserError<'a> = Error<MutexGuard<'a, tree_sitter::Parser>>;
pub type TreeError<'a> = Error<MutexGuard<'a, tree_sitter::Tree>>;

pub struct Parser(Mutex<tree_sitter::Parser>);

impl Parser {
    pub fn new<'a>(lang: tree_sitter::Language) -> Result<Self, ParserError<'a>> {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(lang)?;
        Ok(Self(Mutex::new(parser)))
    }

    pub fn set_language(&self, lang: tree_sitter::Language) -> Result<(), ParserError<'_>> {
        let mut parser = self.lock()?;
        parser.set_language(lang)?;
        Ok(())
    }

    pub fn set_included_ranges(
        &self,
        ranges: &[tree_sitter::Range],
    ) -> Result<(), ParserError<'_>> {
        let mut parser = self.lock()?;
        parser.set_included_ranges(ranges)?;
        Ok(())
    }

    pub fn parse(&self, text: impl AsRef<[u8]>, old_tree: Option<&Tree>) -> Option<Tree> {
        let mut parser = self.0.lock().ok()?;
        let old_tree = if let Some(tree) = old_tree {
            tree.lock().ok()
        } else {
            None
        };
        parser.parse(text, old_tree.as_deref()).map(Tree::new)
    }

    pub fn lock<'a>(&'a self) -> Result<MutexGuard<'a, tree_sitter::Parser>, ParserError<'a>> {
        Ok(self.0.lock()?)
    }
}

pub struct Tree(Mutex<tree_sitter::Tree>);

impl Tree {
    fn new(tree: tree_sitter::Tree) -> Self {
        Self(Mutex::new(tree))
    }

    pub fn root_node(&self) -> Node {
        Node::from_tsnode(&self.lock().unwrap().root_node(), None)
    }

    pub fn pre_walk(&self) -> Vec<Node> {
        let tree = self.lock().unwrap();
        let mut cursor = tree.walk();
        let mut output = vec![];

        'outer: loop {
            output.push(Node::from_tsnode(&cursor.node(), None));
            // Going down the tree
            if cursor.goto_first_child() {
                continue;
            }
            // Going across the tree
            while !cursor.goto_next_sibling() {
                if !cursor.goto_parent() {
                    break 'outer;
                }
            }
        }

        output
    }

    pub fn edit(&self, edit: InputEdit) {
        let mut tree = self.lock().unwrap();
        tree.edit(&edit.into());
    }

    pub fn lock<'a>(&'a self) -> Result<MutexGuard<'a, tree_sitter::Tree>, TreeError<'a>> {
        Ok(self.0.lock()?)
    }
}

pub fn load(env: Env) -> bool {
    resource!(Parser, env);
    resource!(Tree, env);
    true
}
