use syn::token::Struct;
/**
 * find_node.rs
 * Enum for all possible ast types that can be found
 * author: 0xMemoryGrinder
 */

use syn_solidity::{ExprNew, ItemContract, ItemEnum, ItemFunction, VariableDeclaration, ExprCall, SolIdent, VariableDefinition, Type, VarDeclDecl, Expr, SolPath, File, Variant, Item, ItemEvent, ItemError, Modifier, EventParameter};

#[derive(Debug)]
pub enum FoundNode {
    // Contracts
    ContractDefName(ItemContract),
    ContractDefInheritance(ItemContract, Modifier),
    ContractInstantiation(ItemContract, Option<ItemFunction>, ExprNew),


    // Functions
    FunctionDefName(ItemContract, ItemFunction),
    FunctionDefParameterName(ItemContract, ItemFunction, VariableDeclaration, Option<SolIdent>),
    FunctionUsageName(ItemContract, ItemFunction, ExprCall),

    // Properties/Variables/Constants
    PropertyDefName(ItemContract, VariableDefinition, SolIdent),
    ConstantVariableDefName(VariableDefinition, SolIdent),
    VariableDefName(Option<ItemContract>, Option<ItemFunction>, VariableDeclaration, Option<SolIdent>),
    TypeUsage(Option<ItemContract>, Option<ItemFunction>, Option<Expr>, Type),

    // Structs
    StructDefName(Option<ItemContract>, SolIdent),
    StructDefPropertyName(ItemContract, Option<ItemFunction>, VariableDeclaration, Option<SolIdent>),


    // Enum
    EnumDefName(Option<ItemContract>, ItemEnum, SolIdent),
    EnumDefValue(Option<ItemContract>, ItemEnum, Variant, SolIdent),

    // Errors
    ErrorDefName(Option<ItemContract>, ItemError, SolIdent),
    ErrorDefParameter(Option<ItemContract>, ItemError, VariableDeclaration),

    // Events
    EventDefName(ItemContract, ItemEvent, SolIdent),
    EventDefParameter(ItemContract, ItemEvent, EventParameter),

    /// TODO!:
    /// HOW TO DIFFERENTIATE BETWEEN THESE TWO?
    VariableUsageName(Option<ItemContract>, Option<ItemFunction>, Expr, SolIdent),
    StructInstantiation(ItemContract, Option<ItemFunction>, SolIdent),

    // Contracts
    //ContractScope(ItemContract, Option<ItemFunction>, SolPath),

    // Functions
    //FunctionDefParameterType(ItemContract, ItemFunction, VariableDeclaration, Type),

    // Properties
    //PropertyDefType(ItemContract, VariableDefinition, Type),

    // Variables
    //VariableDefType(File,ItemContract, VarDeclDecl, VariableDeclaration, Type),

    // Structs
    //StructUsageName(ItemContract, Option<ItemFunction>, Expr, Type),
    //StructUsageProperty(ItemContract, Option<ItemFunction>, Expr, SolIdent, Type),


    // Errors
    //ErrorUsageName(ItemContract, Option<ItemFunction>, ExprCall, SolIdent),


    // Events
    //EventUsageName(ItemContract, Option<ItemFunction>, Expr, SolIdent),


    //TODO type cast
    // TODO super ast node
}