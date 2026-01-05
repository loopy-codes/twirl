//! Language-agnostic symbol types.

pub use lsp_types::SymbolKind;
use lsp_types::{self};

pub struct Symbol {
    metadata: lsp_types::DocumentSymbol,
    documentation: Option<String>,
}

impl Symbol {
    fn new(metadata: lsp_types::DocumentSymbol) -> Self {
        Self {
            metadata,
            documentation: None,
        }
    }

    fn with_documentation(&mut self, documentation: String) -> &mut Self {
        self.documentation = Some(documentation);
        self
    }
}
