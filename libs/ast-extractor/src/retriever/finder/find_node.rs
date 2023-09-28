/**
 * find_node.rs
 * Enum for all possible ast types that can be found
 * author: 0xMemoryGrinder
 */

use syn_solidity::{ExprNew, ItemContract, ItemEnum, ItemFunction, VariableDeclaration, ExprCall, SolIdent, VariableDefinition, Type, VarDeclDecl, Expr};

pub enum FoundNode {





    // Contracts
    ContractDefName(ItemContract),
    ContractDefInheritance(ItemContract),
    ContractInstantiation(ItemContract, Option<ItemFunction>, ExprNew),
    //TODO : ContractScope(ItemContract, Option<ItemFunction>, ????),

    // Functions
    FunctionDefName(ItemContract, ItemFunction),
    FunctionDefParameter(ItemContract, ItemFunction, VariableDeclaration),
    FunctionDefReturn(ItemContract, ItemFunction, VariableDeclaration),
    FunctionUsageName(ItemContract, ItemFunction, ExprCall),

    // Properties
    PropertyDefName(ItemContract, VariableDefinition, SolIdent),
    PropertyDefType(ItemContract, VariableDefinition, Type),

    // Variables
    VariableDefName(ItemContract, VarDeclDecl, VariableDeclaration, Option<SolIdent>),
    VariableDefType(ItemContract, VarDeclDecl, VariableDeclaration, Type),
    VariableUsageName(Option<ItemContract>, Option<ItemFunction>, Expr, SolIdent),







    FileEnum(ItemEnum),
    ContractEnum((ItemContract, ItemEnum))

}