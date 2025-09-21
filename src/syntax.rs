use tree_sitter::{Node, Tree};

pub fn functions<'a>(tree: &'a Tree) -> Vec<Node<'a>> {
    let root = tree.root_node();
    let mut cursor = root.walk();
    return root
        .children(&mut cursor)
        .filter(|node| node.kind() == "function_definition") // NOTE: not guaranteed to be the right label
        .collect();
}

pub fn docstring<'a>(node: &'a Node) -> Option<String> {
    // if (curr_node.type=='expression_statement' and len(curr_node.children)==1 and (curr_node.children[0].type=='string'):
    if node.kind() == "expression_statement" && node.child_count() == 1 {
        let child = node.child(0).unwrap();
        if child.kind() == "string" {
            return Some(child.to_string());
        }
    }
    return None;
}
