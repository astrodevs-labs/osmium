/**
 * finder.rs
 * Function to retrieve contract nodes from position
 * author: 0xMemoryGrinder
 */
use syn_solidity::*;
use proc_macro2::LineColumn;
mod find_node;



pub struct Position {
    line: usize,
    char: usize,
}

impl Position {
    pub fn new() -> Self {
        Self {
            line: 0,
            char: 0,
        }
    }
    pub fn from(line: usize, char: usize) -> Self {
        Self {
            line,
            char,
        }
    }

}

pub struct FoundNode {
    node_type: String,
    node: Box<dyn Spanned>,
    sub_span: Option<Box<dyn Spanned>>,
}


impl FoundNode {
    pub fn new(node_type: String, node: Box<dyn Spanned>, sub_span: Option<Box<dyn Spanned>>) -> Self {
        Self {
            node,
            node_type,
            sub_span,
        }
    }
}
struct FinderVisitor {
    found: Option<FoundNode>,
    to_find: Position,
}


impl FinderVisitor {

    pub fn new(pos: Position) -> Self {
        Self {
            found: None,
            to_find: pos,
        }
    }
    pub fn check_matching(&mut self, node: Box<dyn Spanned>, node_type: String) -> bool {
        let start_pos = node.span().start();
        let end_pos = node.span().end();
        if self._is_in_range(start_pos, end_pos) {
            self.found = Some(FoundNode::new(
                node_type,
                node,
                None
            ));
            return true;
        }
        return false;
    }

    fn _is_in_range(&self, start: LineColumn, end: LineColumn) -> bool {
        if self.to_find.char >= start.column && self.to_find.line <= start.line
            && self.to_find.char <= end.column && self.to_find.line <= end.line {
            return true;
        }
        return false;
    }

    pub fn check_matching_sub_span(&mut self, node: Box<dyn Spanned>, node_type: String, sub_span: Option<Box<dyn Spanned>>) -> bool{
        let start_pos = sub_span.span().start();
        let end_pos = sub_span.span().end();
        if self._is_in_range(start_pos, end_pos) {
            self.found = Some(FoundNode::new(
                node_type,
                node,
                sub_span
            ));
            return true;
        }
        return false;
    }

    pub fn stmts_finder(&mut self, block: Block) -> (bool, Option<Stmt>) {
        let mut found = false;

        for stmt in &block.stmts {
            if found {
                break
            }
            match stmt {
                Stmt::VarDecl(node) => {
                    match &*node.declaration {
                        VarDeclDecl::VarDecl(decl) => {
                            match &decl.ty {
                                Type::Address(span, payable) => {
                                    if self.check_matching_sub_span(Box::new(decl.clone()), String::from("address"), Some(Box::new(span.clone()))) {
                                        break
                                    }
                                }
                                Type::String(span) => {
                                    if self.check_matching_sub_span(Box::new(decl.clone()), String::from("string"), Some(Box::new(span.clone()))) {
                                        break
                                    }
                                }
                                Type::Bytes(span) => {
                                    if self.check_matching_sub_span(Box::new(decl.clone()), String::from("bytes"), Some(Box::new(span.clone()))) {
                                        break

                                    }
                                }
                                Type::Custom(span) => {
                                    if self.check_matching_sub_span(Box::new(decl.clone()), String::from("custom"), Some(Box::new(span.clone()))) {
                                        break
                                    }

                                }
                                _ => {}
                            }
                            if found {
                                break
                            }

                        }
                        _ => {}
                    }
                    if found {
                        break
                    }

                },
                _ => {}
            }
        }
        return (false, None);
    }

}

impl<'ast> Visit<'ast> for FinderVisitor {
    fn visit_item_function(&mut self, function: &'ast ItemFunction) {
        let mut found: bool = false;

        match &function.body {
            FunctionBody::Block(block) => {
                for stmt in &block.stmts {
                    println!("stmt: {:?}", stmt);
                    if found {
                        break
                    }
                    match stmt {
                        Stmt::VarDecl(node) => {
                            match &*node.declaration {
                                VarDeclDecl::VarDecl(decl) => {
                                    match &decl.ty {
                                        Type::Address(span, payable) => {
                                            if self.check_matching_sub_span(Box::new(decl.clone()), String::from("address"), Some(Box::new(span.clone()))) {
                                                break
                                            }
                                        }
                                        Type::String(span) => {
                                            if self.check_matching_sub_span(Box::new(decl.clone()), String::from("string"), Some(Box::new(span.clone()))) {
                                                break
                                            }
                                        }
                                        Type::Bytes(span) => {
                                            if self.check_matching_sub_span(Box::new(decl.clone()), String::from("bytes"), Some(Box::new(span.clone()))) {
                                                break

                                            }
                                        }
                                        Type::Custom(span) => {
                                            if self.check_matching_sub_span(Box::new(decl.clone()), String::from("custom"), Some(Box::new(span.clone()))) {
                                                break
                                            }

                                        }
                                        _ => {}
                                    }
                                    if found {
                                        break
                                    }

                                }
                                _ => {}
                            }
                            if found {
                                break
                            }

                        },
                        _ => {}
                    }
                }
            },
            _ => {}
        }
        syn_solidity::visit::visit_item_function(self, function);
    }

}


pub fn retrieve_node_from_position(ast: &syn_solidity::File, pos: Position) -> Option<FoundNode> {
    let mut visitor = FinderVisitor::new(pos);
    visitor.visit_file(ast);
    visitor.found
}


#[cfg(test)]
mod tests {
    use proc_macro2::TokenStream;

    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::str::FromStr;

    #[test]
    fn test_retrieve_node_type_decl_string() {
        let source = String::from("pragma solidity ^0.8.0;\
        abstract contract One {
    uint storedData;
    function set(uint x) public {
        storedData = x;
        string test2;
    }

    function get() public view returns (uint) {
        return storedData;
    }
}");
        let tokens = TokenStream::from_str(source.as_str()).unwrap();
        let ast = syn_solidity::parse2(tokens).unwrap();
        let res = retrieve_node_from_position(&ast, Position::from(5, 8));
        match &res {
            Some(node) => {
                match &node.sub_span {
                    Some(span) => {assert!(true)}
                    _ => {}
                }
                assert_eq!(node.node_type, "string");

            }
            _ => {assert!(false)}
        }

    }
}
