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

    pub fn queries(&self) -> Vec<(Atom, &'static str)> {
        match self {
            Self::Css => css_queries(),
            Self::Elixir => elixir_queries(),
            Self::EmbeddedTemplate => embedded_template_queries(),
            Self::Erlang => erlang_queries(),
            Self::Gleam => gleam_queries(),
            Self::Html => html_queries(),
            Self::Javascript => javascript_queries(),
            Self::Sql => sql_queries(),
            Self::Typescript => typescript_queries(),
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

#[cfg(feature = "css")]
fn css_queries() -> Vec<(Atom, &'static str)> {
    vec![(
        crate::atoms::highlights(),
        tree_sitter_css::HIGHLIGHTS_QUERY,
    )]
}

#[cfg(not(feature = "css"))]
fn css_queries() -> Vec<(Atom, &'static str)> {
    vec![]
}

#[cfg(feature = "elixir")]
fn elixir_queries() -> Vec<(Atom, &'static str)> {
    vec![
        (
            crate::atoms::highlights(),
            tree_sitter_elixir::HIGHLIGHTS_QUERY,
        ),
        (crate::atoms::tags(), tree_sitter_elixir::TAGS_QUERY),
    ]
}

#[cfg(not(feature = "elixir"))]
fn elixir_queries() -> Vec<(Atom, &'static str)> {
    vec![]
}

#[cfg(feature = "embedded-template")]
fn embedded_template_queries() -> Vec<(Atom, &'static str)> {
    vec![(
        crate::atoms::highlights(),
        tree_sitter_embedded_template::HIGHLIGHT_QUERY,
    )]
}

#[cfg(not(feature = "embedded-template"))]
fn embedded_template_queries() -> Vec<(Atom, &'static str)> {
    vec![]
}

fn erlang_queries() -> Vec<(Atom, &'static str)> {
    vec![]
}

#[cfg(feature = "gleam")]
fn gleam_queries() -> Vec<(Atom, &'static str)> {
    vec![
        (
            crate::atoms::highlights(),
            tree_sitter_gleam::HIGHLIGHTS_QUERY,
        ),
        (crate::atoms::locals(), tree_sitter_gleam::LOCALS_QUERY),
        (crate::atoms::tags(), tree_sitter_gleam::TAGS_QUERY),
    ]
}

#[cfg(not(feature = "gleam"))]
fn gleam_queries() -> Vec<(Atom, &'static str)> {
    vec![]
}

#[cfg(feature = "html")]
fn html_queries() -> Vec<(Atom, &'static str)> {
    vec![
        (
            crate::atoms::highlights(),
            tree_sitter_html::HIGHLIGHTS_QUERY,
        ),
        (crate::atoms::injection(), tree_sitter_html::INJECTIONS_QUERY),
    ]
}

#[cfg(not(feature = "html"))]
fn html_queries() -> Vec<(Atom, &'static str)> {
    vec![]
}

#[cfg(feature = "javascript")]
fn javascript_queries() -> Vec<(Atom, &'static str)> {
    vec![
        (
            crate::atoms::highlights(),
            tree_sitter_javascript::HIGHLIGHT_QUERY,
        ),
        (
            crate::atoms::injection(),
            tree_sitter_javascript::INJECTION_QUERY,
        ),
        (
            crate::atoms::jsx(),
            tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
        ),
        (crate::atoms::locals(), tree_sitter_javascript::LOCALS_QUERY),
        (crate::atoms::tags(), tree_sitter_javascript::TAGGING_QUERY),
    ]
}

#[cfg(not(feature = "javascript"))]
fn javascript_queries() -> Vec<(Atom, &'static str)> {
    vec![]
}

fn sql_queries() -> Vec<(Atom, &'static str)> {
    vec![]
}

#[cfg(feature = "typescript")]
fn typescript_queries() -> Vec<(Atom, &'static str)> {
    vec![
        (
            crate::atoms::highlights(),
            tree_sitter_typescript::HIGHLIGHT_QUERY,
        ),
        (crate::atoms::locals(), tree_sitter_typescript::LOCALS_QUERY),
        (crate::atoms::tags(), tree_sitter_typescript::TAGGING_QUERY),
    ]
}

#[cfg(not(feature = "typescript"))]
fn typescript_queries() -> Vec<(Atom, &'static str)> {
    vec![]
}
