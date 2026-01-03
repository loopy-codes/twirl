use loom::{languages, queries};
use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator};

#[test]
fn test_python_language_detection() {
    // Test that Python files are properly detected
    let python_lang = languages::language("py");
    assert!(python_lang.is_some(), "Python language should be supported");
}

#[test]
fn test_supported_extensions_includes_python() {
    // Test that Python is in the list of supported extensions
    let extensions = languages::supported_extensions();
    assert!(
        extensions.contains(&"py".to_string()),
        "Python (.py) should be in supported extensions"
    );
}

#[test]
fn test_parse_simple_python_code() {
    // Test parsing actual Python code
    let python_lang = languages::language("py").expect("Python language should be available");

    let mut parser = Parser::new();
    parser
        .set_language(python_lang)
        .expect("Failed to set Python language");

    let source_code = r#"
def hello_world():
    print("Hello, World!")
    return 42

if __name__ == "__main__":
    result = hello_world()
"#;

    let tree = parser
        .parse(source_code, None)
        .expect("Failed to parse Python code");
    let root_node = tree.root_node();

    // Verify we got a valid parse tree
    assert_eq!(root_node.kind(), "module", "Root node should be a module");
    assert!(
        root_node.child_count() > 0,
        "Root node should have children"
    );

    // Check that we can find function definitions
    let mut cursor = root_node.walk();

    let functions_count = root_node
        .children(&mut cursor)
        .map(|child| {
            if child.kind() == "function_definition" {
                1
            } else {
                0
            }
        })
        .count();

    assert!(
        functions_count > 0,
        "Should find at least one function definition"
    );
}

#[test]
fn test_parse_python_with_syntax_error() {
    // Test parsing Python code with syntax errors
    let python_lang = languages::language("py").expect("Python language should be available");

    let mut parser = Parser::new();
    parser
        .set_language(python_lang)
        .expect("Failed to set Python language");

    let invalid_code = r#"
def broken_function(
    print("This is missing a closing parenthesis"
    return "incomplete"
"#;

    let tree = parser
        .parse(invalid_code, None)
        .expect("Parser should still return a tree");
    let root_node = tree.root_node();

    // Even with syntax errors, we should get some kind of parse tree
    assert!(
        root_node.has_error(),
        "Parse tree should indicate there are errors"
    );
}

#[test]
fn test_python_docstring_queries() {
    // Get Python language and query
    let python_lang = languages::language("py").expect("Python language should be available");
    let query_source = queries::docstring_query("py").expect("Python docstring query should exist");

    let query = Query::new(python_lang, query_source).expect("Should be able to create query");
    let mut parser = Parser::new();
    parser
        .set_language(python_lang)
        .expect("Failed to set Python language");

    let source_code = r#"
"""
This is a module-level docstring.
It describes the purpose of this Python module.
"""

def documented_function():
    """
    This function has a docstring that describes what it does.
    It demonstrates our ability to capture function-level docstrings.
    """
    return "Hello, documentation!"

class DocumentedClass:
    """
    This class has a docstring.
    It shows we can capture class-level docstrings too.
    """
    pass

# This is a regular comment
# This is a documentation comment that we should capture
"#;

    let tree = parser
        .parse(source_code, None)
        .expect("Failed to parse Python code");

    let mut query_cursor = QueryCursor::new();

    let mut docstrings = Vec::new();
    let mut matches = query_cursor.matches(&query, tree.root_node(), source_code.as_bytes());
    while let Some(m) = matches.next() {
        let capture = m.captures[0];
        docstrings.push(&source_code[capture.node.byte_range()]);
    }

    // We should find 4 docstrings:
    // 1. Module docstring
    // 2. Function docstring
    // 3. Class docstring
    // 4. Documentation comment
    assert_eq!(docstrings.len(), 4, "Should find exactly 4 docstrings");

    // Verify content of docstrings
    assert!(
        docstrings
            .iter()
            .any(|s| s.contains("module-level docstring")),
        "Should find module docstring"
    );
    assert!(
        docstrings
            .iter()
            .any(|s| s.contains("function has a docstring")),
        "Should find function docstring"
    );
    assert!(
        docstrings
            .iter()
            .any(|s| s.contains("class has a docstring")),
        "Should find class docstring"
    );
    assert!(
        docstrings
            .iter()
            .any(|s| s.contains("documentation comment that we should capture")),
        "Should find documentation comment"
    );
}
