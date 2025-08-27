use tree_sitter::Language;

use tree_sitter_c;
use tree_sitter_cpp;
use tree_sitter_java;
use tree_sitter_javascript;
use tree_sitter_julia;
use tree_sitter_python;
use tree_sitter_rust;
use tree_sitter_typescript;

pub static LANGUAGES: &[&str] = &[
    "c", "h", "hpp", "cpp", "cc", "jl", "py", "rs", "java", "js", "ts",
];

pub fn language(file_extension: &str) -> Option<Language> {
    match file_extension {
        "c" => Some(tree_sitter_c::LANGUAGE.into()),
        "h" => Some(tree_sitter_c::LANGUAGE.into()),
        "hpp" => Some(tree_sitter_cpp::LANGUAGE.into()),
        "cpp" => Some(tree_sitter_cpp::LANGUAGE.into()),
        "cc" => Some(tree_sitter_cpp::LANGUAGE.into()),
        "jl" => Some(tree_sitter_julia::LANGUAGE.into()),
        "py" => Some(tree_sitter_python::LANGUAGE.into()),
        "rs" => Some(tree_sitter_rust::LANGUAGE.into()),
        "java" => Some(tree_sitter_java::LANGUAGE.into()),
        "js" => Some(tree_sitter_javascript::LANGUAGE.into()),
        "ts" => Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn languages() {
        // unsupported extensions
        assert_eq!(language(""), None);

        // supported extensions
        for lang in LANGUAGES {
            assert_ne!(language(lang), None);
        }
    }
}
