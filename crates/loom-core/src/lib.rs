use std::fmt;
use std::fs;
use std::io;
use std::path::PathBuf;

use arborium::tree_sitter::{Language, Parser, Tree};

/// Errors that can occur in loom-core operations.
#[derive(Debug)]
pub enum Error {
    /// File does not exist at the specified path.
    FileNotFound(PathBuf),
    /// Failed to read file contents.
    IoError(io::Error),
    /// Failed to convert path to string.
    PathConversionError(PathBuf),
    /// Failed to detect language from file path.
    LanguageDetectionError(String),
    /// Language is not supported.
    UnsupportedLanguage(String),
    /// Failed to set language on parser.
    ParserLanguageError(arborium::tree_sitter::LanguageError),
    /// Failed to parse source code.
    ParseError,
    /// Symbol validation error.
    InvalidSymbol(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FileNotFound(path) => write!(f, "file not found: {}", path.display()),
            Error::IoError(err) => write!(f, "I/O error: {}", err),
            Error::PathConversionError(path) => {
                write!(f, "failed to convert path to string: {}", path.display())
            }
            Error::LanguageDetectionError(msg) => write!(f, "failed to detect language: {}", msg),
            Error::UnsupportedLanguage(lang) => write!(f, "unsupported language: {}", lang),
            Error::ParserLanguageError(err) => write!(f, "failed to set parser language: {}", err),
            Error::ParseError => write!(f, "failed to parse source code"),
            Error::InvalidSymbol(msg) => write!(f, "invalid symbol: {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IoError(err) => Some(err),
            Error::ParserLanguageError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<arborium::tree_sitter::LanguageError> for Error {
    fn from(err: arborium::tree_sitter::LanguageError) -> Self {
        Error::ParserLanguageError(err)
    }
}

/// Given a language name, return the corresponding `Language` instance if one is available.
/// Note that this may be replaced by `get_language` when [#105] merges.
///
/// [#105]: https://github.com/bearcove/arborium/issues/105
fn language_from_str(language: &str) -> Option<Language> {
    match language {
        // FIXME support other languages
        "rust" => Some(arborium::lang_rust::language().into()),
        "python" => Some(arborium::lang_python::language().into()),
        _ => None,
    }
}

/// A single source file.
pub struct SourceCode {
    /// The code content.
    content: String,
    /// The language of the source code; only one language is supported.
    language: Language,
}

impl SourceCode {
    pub fn new(content: String, language: Language) -> Self {
        SourceCode { content, language }
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

        let language = language_from_str(language_name)
            .ok_or(Error::UnsupportedLanguage(language_name.to_string()))?;

        let code = SourceCode { content, language };

        Ok(code)
    }

    pub fn into_tree(self) -> Result<Tree, Error> {
        let mut parser = Parser::new();
        parser.set_language(&self.language)?;

        parser.parse(&self.content, None).ok_or(Error::ParseError)
    }
}

/// All types of symbols.
#[derive(Hash, Debug)]
pub enum SymbolKind {
    /// All types, including classes and structures.
    Type,
    /// Instances of a given type.
    Instance,
    /// Functions, which may or may not be tied to any given type.
    Function,
    /// Interfaces, including abstract and concrete class definitions, trait definitions, etc.
    Interface,
    /// Modules, including implicit modules i.e. C header/source pairs.
    Module,
}

/// A single symbol.
#[derive(Hash, Debug)]
pub struct Symbol {
    /// A name unique to the parent namespace.
    name: String,
    /// The namespace which owns the symbol.
    namespace: String,
    /// What the symbol is.
    kind: SymbolKind,
    /// Documentation for the symbol, if any.
    documentation: Option<String>,
}

impl Symbol {
    pub fn new(name: String, namespace: String, kind: SymbolKind) -> Result<Self, Error> {
        if name.is_empty() || namespace.is_empty() {
            return Err(Error::InvalidSymbol(
                "both `name` and `namespace` must be non-empty".to_string(),
            ));
        }
        let symbol = Symbol {
            name,
            namespace,
            kind,
            documentation: None,
        };
        Ok(symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbols() {}
}
