/**
 * finder.rs
 * Function to retrieve contract nodes from position
 * author: 0xSwapFeeder
 */
use syn_solidity::*;
use proc_macro2::LineColumn;
use syn::ExprLit;
use syn_solidity::kw::contract;
use syn_solidity::visit::visit_variable_declaration;
use crate::retriever::finder::find_node::FoundNode;

mod find_node;

macro_rules! is_in_range {
    ($start:expr, $end:expr, $pos:expr) => {
        $pos.char >= $start.column && $pos.line <= $start.line
            && $pos.char <= $end.column && $pos.line <= $end.line
    };
}

pub struct Position {
    line: usize,
    char: usize,
}

impl Position {
    pub fn new(line: usize, char: usize) -> Self {
        Self {
            line,
            char,
        }
    }

}

impl Default for Position {
    fn default() -> Self {
        Self {
            line: 0,
            char: 0,
        }
    }
}

struct FinderVisitor {
    current_contract: Option<Box<ItemContract>>,
    current_function: Option<Box<ItemFunction>>,
    current_property: Option<Box<VariableDefinition>>,
    current_variable: Option<Box<VariableDeclaration>>,
    current_enum: Option<Box<ItemEnum>>,
    current_struct: Option<Box<ItemStruct>>,
    current_error: Option<Box<ItemError>>,
    current_event: Option<Box<ItemEvent>>,
    current_expr: Option<Box<Expr>>,
    current_stmt: Option<Box<Stmt>>,
    found: Option<FoundNode>,
    to_find: Position,
}


impl FinderVisitor {

    pub fn new(pos: Position) -> Self {
        Self {
            current_contract: None,
            current_function: None,
            current_property: None,
            current_variable: None,
            current_enum: None,
            current_struct: None,
            current_error: None,
            current_event: None,
            current_expr: None,
            current_stmt: None,
            found: None,
            to_find: pos,
        }
    }

    fn check_inheritance_matching(&mut self, contract: &ItemContract) -> bool {
        if let Some(inheritance) = &contract.inheritance {
            if is_in_range!(inheritance.span().start(), inheritance.span().end(), self.to_find) {
                for inherit in inheritance.inheritance {
                    if is_in_range!(inherit.span().start(), inherit.span().end(), self.to_find) {
                        self.found = Some(FoundNode::ContractDefInheritance(contract.clone(), inherit));
                        return true;
                    }
                }
            }
        }
        return false;
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

}

impl<'ast> Visit<'ast> for FinderVisitor {
    fn visit_expr_call(&mut self, i: &'ast ExprCall) {
        todo!()
    }

    fn visit_variable_declaration(&mut self, var: &'ast VariableDeclaration) {
        if is_in_range!(var.span().start(), var.span().end(), self.to_find) {
            self.current_variable = Some(Box::new(var.clone()));
            if is_in_range!(var.name.span().start(), var.name.span().end(), self.to_find) {
                    if let VarDeclDecl();
                    self.found = Some(FoundNode::VariableDefName(*self.current_contract.clone(), *self.current_function, var.clone()));
                    return;
                }
            }
            visit_variable_declaration(self, var);
    }

    fn visit_parameter_list(&mut self, params: &'ast ParameterList) {
        todo!()
    }

    fn visit_stmt_var_decl(&mut self, stmt_var_decl: &'ast StmtVarDecl) {
        todo!("VariableDefName and variableDefType")
    }

    fn visit_variable_definition(&mut self, var: &'ast VariableDefinition) {
        if is_in_range!(var.span().start(), var.span().end(), self.to_find) {
            self.current_property = Some(Box::new(var.clone()));
            if is_in_range!(var.ty.span().start(), var.ty.span().end(), self.to_find) {

                self.found = Some(FoundNode::TypeUsage(*self.current_contract,self.current_function,None, *var.ty));
                return;
            }
            if is_in_range!(var.name.span().start(), var.name.span().end(), self.to_find) {
                if self.current_contract.is_none() {
                    self.found = Some(FoundNode::ConstantVariableDefName(var.clone(), *var.name))
                } else {
                    self.found = Some(FoundNode::PropertyDefName(*self.current_contract,var.clone(), *var.name));
                }
                return;
            }
            visit::visit_variable_definition(self, var);
        }
    }

    fn visit_item_contract(&mut self, contract: &'ast ItemContract) {

        if is_in_range!(contract.span().start(), contract.span().end(), self.to_find) {
            self.current_contract = Some(Box::new(contract.clone()));
            if is_in_range!(contract.name.span().start(), contract.name.span().end(), self.to_find) {
                self.found = Some(FoundNode::ContractDefName(contract.clone()));
                return;
            }
            if self.check_inheritance_matching(contract) {
                return;
            }

            visit::visit_item_contract(self, contract);
        }
    }

    fn visit_item_function(&mut self, function: &'ast ItemFunction) {
        if is_in_range!(function.span().start(), function.span().end(), self.to_find) {
            self.current_function = Some(Box::new(function.clone()));
            if is_in_range!(function.name.span().start(), function.name.span().end(), self.to_find) {
                self.found = Some(FoundNode::FunctionDefName(*self.current_contract.clone(), function.clone()));
                return;
            }
            if let Some(rturn) =  &function.returns {
                rturn.returns.iter().for_each(|r| {
                    if is_in_range!(r.ty.span().start(), r.ty.span().end(), self.to_find) {
                        self.found = Some(FoundNode::TypeUsage(*self.current_contract, Some(function.clone()), None, r.ty.clone()));
                        return;
                    }
                })
            }
            for param in &function.arguments {
                if is_in_range!(param.name.span().start(), param.name.span().end(), self.to_find) {
                    self.found = Some(FoundNode::FunctionDefParameterName(*self.current_contract, function.clone(), param.clone(), *param.name));
                    return;
                }
            }
            visit::visit_item_function(self, function);
        }
    }

    fn visit_stmt(&mut self, stmt: &'ast Stmt) {
        self.current_stmt = Some(Box::new(stmt.clone()));
        visit::visit_stmt(self, stmt);
    }

    //TODO: Found Limitation: cannot check parameter list of a new expr
    // Therefore we can not goto or list_ref any variable used in a new expr
    fn visit_expr_new(&mut self, new: &'ast ExprNew) {
        if is_in_range!(new.span().start(), new.span().end(), self.to_find) {
            self.current_expr = Some(Box::new(Expr::New(new.clone())));
            self.found = Some(FoundNode::ContractInstantiation(*self.current_contract.clone(), *self.current_function.clone(), new.clone()));
        }
    }

}


pub fn retrieve_node_from_position(ast: &syn_solidity::File, pos: Position) -> Option<find_node::FoundNode> {
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
        let res = retrieve_node_from_position(&ast, Position::new(5, 8));
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
