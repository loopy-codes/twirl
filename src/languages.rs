use once_cell::sync::Lazy;
use std::collections::HashMap;
use tree_sitter::Language;
use tree_sitter_c;
use tree_sitter_cpp;
use tree_sitter_java;
use tree_sitter_javascript;
use tree_sitter_julia;
use tree_sitter_python;
use tree_sitter_rust;
use tree_sitter_typescript;

static LANGUAGE_EXTENSIONS: Lazy<HashMap<&'static str, Language>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // C/C++
    m.insert("c", tree_sitter_c::LANGUAGE.into());
    m.insert("h", tree_sitter_c::LANGUAGE.into());
    m.insert("hpp", tree_sitter_cpp::LANGUAGE.into());
    m.insert("cpp", tree_sitter_cpp::LANGUAGE.into());
    m.insert("cc", tree_sitter_cpp::LANGUAGE.into());

    // Other languages
    m.insert("jl", tree_sitter_julia::LANGUAGE.into());
    m.insert("py", tree_sitter_python::LANGUAGE.into());
    m.insert("rs", tree_sitter_rust::LANGUAGE.into());
    m.insert("java", tree_sitter_java::LANGUAGE.into());
    m.insert("js", tree_sitter_javascript::LANGUAGE.into());
    m.insert("ts", tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into());

    return m;
});

pub fn supported_extensions() -> Vec<String> {
    return LANGUAGE_EXTENSIONS
        .keys()
        .cloned()
        .map(|key| key.to_string())
        .collect();
}

pub fn language(file_extension: &str) -> Option<&Language> {
    return LANGUAGE_EXTENSIONS.get(file_extension);
}

// Unit Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_languages() {
        // unsupported extensions
        assert!(language("").is_none());

        // supported extensions
        for ext in supported_extensions() {
            assert!(language(ext.as_str()).is_some());
        }
    }
}
