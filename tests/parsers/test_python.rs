use pearl::languages;
use tree_sitter::{Parser, Tree};

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
    let mut found_function = false;

    for child in root_node.children(&mut cursor) {
        if child.kind() == "function_definition" {
            found_function = true;
            break;
        }
    }

    assert!(
        found_function,
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
