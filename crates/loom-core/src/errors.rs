use std::fmt;
use std::io;
use std::path::PathBuf;

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
