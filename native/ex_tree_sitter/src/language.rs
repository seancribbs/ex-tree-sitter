use rustler::*;

#[derive(NifUnitEnum)]
pub enum Language {
    Css,
    Elixir,
    EmbeddedTemplate,
    Erlang,
    Gleam,
    Html,
    Javascript,
    Sql,
    Typescript,
}

impl Language {
    pub fn supported(&self) -> bool {
        match self {
            Self::Css => cfg!(feature = "css"),
            Self::Elixir => cfg!(feature = "elixir"),
            Self::EmbeddedTemplate => cfg!(feature = "embedded-template"),
            Self::Erlang => cfg!(feature = "erlang"),
            Self::Gleam => cfg!(feature = "gleam"),
            Self::Html => cfg!(feature = "html"),
            Self::Javascript => cfg!(feature = "javascript"),
            Self::Sql => cfg!(feature = "sql"),
            Self::Typescript => cfg!(feature = "typescript"),
        }
    }

    pub fn get_language(&self) -> Option<tree_sitter::Language> {
        match self {
            Self::Css => get_css(),
            Self::Elixir => get_elixir(),
            Self::EmbeddedTemplate => get_embedded_template(),
            Self::Erlang => get_erlang(),
            Self::Gleam => get_gleam(),
            Self::Html => get_html(),
            Self::Javascript => get_javascript(),
            Self::Sql => get_sql(),
            Self::Typescript => get_typescript(),
        }
    }
}

macro_rules! impl_get_lang {
    ($name:ident, $feat:literal, $call:expr) => {
        #[cfg(feature = $feat)]
        fn $name() -> Option<tree_sitter::Language> {
            Some($call)
        }

        #[cfg(not(feature = $feat))]
        fn $name() -> Option<tree_sitter::Language> {
            None
        }
    };
}

impl_get_lang!(get_css, "css", tree_sitter_css::language());
impl_get_lang!(get_elixir, "elixir", tree_sitter_elixir::language());
impl_get_lang!(
    get_embedded_template,
    "embedded-template",
    tree_sitter_embedded_template::language()
);
impl_get_lang!(get_erlang, "erlang", tree_sitter_erlang::language());
impl_get_lang!(get_gleam, "gleam", tree_sitter_gleam::language());
impl_get_lang!(get_html, "html", tree_sitter_html::language());
impl_get_lang!(
    get_javascript,
    "javascript",
    tree_sitter_javascript::language()
);
impl_get_lang!(get_sql, "sql", tree_sitter_sql::language());
impl_get_lang!(
    get_typescript,
    "typescript",
    tree_sitter_typescript::language()
);
