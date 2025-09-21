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

pub fn language_from_ext(file_extension: &str) -> Option<&Language> {
    return LANGUAGE_EXTENSIONS.get(file_extension);
}

pub fn language_from_path(filepath: &std::path::Path) -> Option<&Language> {
    let file_extension = filepath.extension()?.to_str()?;
    language_from_ext(file_extension)
}

// Python Module

pub mod py {

    use pyo3::prelude::*;
    use pyo3::types::PyList;

    /// Returns a list of supported file extensions
    #[pyfunction]
    fn supported_extensions(py: Python<'_>) -> PyResult<Bound<'_, PyList>> {
        let exts = super::supported_extensions();
        let list = PyList::new(py, exts)?;
        return Ok(list);
    }

    /// A Python module implemented in Rust.
    #[pymodule]
    fn languages(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(supported_extensions, m)?)?;
        Ok(())
    }
}

// Unit Tests

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_languages() {
        // unsupported extensions
        assert!(language_from_ext("").is_none());

        // supported extensions
        for ext in supported_extensions() {
            assert!(language_from_ext(ext.as_str()).is_some());
        }
    }

    #[test]
    fn test_languages_from_paths() {
        // invalid filepaths
        assert!(language_from_path(Path::new("foo")).is_none());

        // valid filepaths
        supported_extensions().into_iter().for_each(|ext| {
            let path = format!("test.{}", ext.as_str());
            let path = Path::new(path.as_str());
            assert!(language_from_path(path).is_some());
        });
    }
}
