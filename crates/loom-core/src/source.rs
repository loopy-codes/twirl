//! Types for tracking source code files with `tree_sitter`.
//!
//! Currently, mutation is not supported.

use crate::Error;

use std::fs;
use std::path::PathBuf;

use arborium::get_language;
use arborium::tree_sitter::{Language, Parser, Tree};

/// A single source file.
pub struct SourceCode {
    /// The code content.
    pub content: String,
    /// The tree associated with the code content.
    pub tree: Tree,
    /// The language of the source code; only one language is supported.
    pub language: Language,
    /// The `tree_sitter` parser for the source code language.
    pub parser: Parser,
}

impl SourceCode {
    pub fn new(content: String, language: Language) -> Self {
        let mut parser = Parser::new();
        parser.set_language(&language).unwrap();

        let tree = parser.parse(&content, None).unwrap();

        SourceCode {
            content,
            tree,
            language,
            parser,
        }
    }

    pub fn from_file(path: PathBuf) -> Result<Self, Error> {
        if !path.exists() {
            return Err(Error::FileNotFound(path));
        }

        let content = fs::read_to_string(&path)?;

        let path_str = path
            .to_str()
            .ok_or(Error::PathConversionError(path.clone()))?;

        let language_name = arborium::detect_language(path_str)
            .ok_or(Error::LanguageDetectionError(path_str.to_string()))?;

        let language = get_language(language_name)
            .ok_or(Error::UnsupportedLanguage(language_name.to_string()))?;

        let code = SourceCode::new(content, language);

        Ok(code)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse() {
        let path = PathBuf::from(env!("CARGO_WORKSPACE_DIR")).join(file!());
        let source = SourceCode::from_file(path).unwrap();

        let sexp = source.tree.root_node().to_sexp();
        assert!(!sexp.is_empty());
    }
}
