use std::fmt::Error;
use ast_extractor::retriever::{Position, retrieve_node_from_position};
use ast_extractor::retriever::finder::{FoundNode};
use ast_extractor::{ItemContract, ItemFunction, ItemStruct, Type, Visit, visit};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RetrieveDefinitionError {
    Extract(#[from] ast_extractor::extract::ExtractError)
}

pub struct DefinitionFinder {
    pos: Position,
    definitions: Vec<FoundNode>
}

impl <'ast>Visit <'ast> for DefinitionFinder {
    fn visit_item_contract(&mut self, contract: &'ast ItemContract) {
        if is_in_range!(self.pos, contract) {
            visit::visit_item_contract(self, contract);
        }
    }



    fn visit_item_struct(&mut self, strukt: &'ast ItemStruct) {

    }

}


fn retreive_definition_from_pos(file: &str, files: Vec<&str>, pos: Position) -> Result<FoundNode, Error> {
    let fileAst = ast_extractor::extract::extract_ast_from_content(file)?;
    let res = retrieve_node_from_position(&fileAst, pos);

    if let Some(node) = res {
        match &node {
            FoundNode::TypeUsage(_, _, _, Type::Custom(t)) => {
                Ok(node)
            }
            FoundNode::IdentUsageCall(_, _, _) => {

            }
            FoundNode::IdentUsageName(_, _, _, _) => {}
            _ => {Ok(node)}
        }
    } else {
    }
}




uint256 myNewVariable = MyContract.constVar;