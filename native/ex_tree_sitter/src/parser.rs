use rustler::*;
use std::ops::Deref;
use std::sync::Mutex;

pub struct Parser(Mutex<tree_sitter::Parser>);

impl Parser {
    pub fn new(lang: tree_sitter::Language) -> Result<Self, tree_sitter::LanguageError> {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(lang)?;
        Ok(Self(Mutex::new(parser)))
    }

    pub fn parse(&self, text: impl AsRef<[u8]>) -> Option<Tree> {
        self.0
            .lock()
            .ok()
            .and_then(|mut p| p.parse(text, None))
            .map(Tree::new)
    }
}

pub struct Tree(Mutex<tree_sitter::Tree>);

impl Deref for Tree {
    type Target = Mutex<tree_sitter::Tree>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Tree {
    fn new(tree: tree_sitter::Tree) -> Self {
        Self(Mutex::new(tree))
    }
}

// pub struct Node<'a>(tree_sitter::Node<'a>);

pub fn load(env: Env) -> bool {
    resource!(Parser, env);
    resource!(Tree, env);
    true
}
