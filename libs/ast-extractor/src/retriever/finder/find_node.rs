use syn::token::Struct;
/**
 * find_node.rs
 * Enum for all possible ast types that can be found
 * author: 0xMemoryGrinder
 */

use syn_solidity::{ExprNew, ItemContract, ItemEnum, ItemFunction, VariableDeclaration, ExprCall, SolIdent, VariableDefinition, Type, VarDeclDecl, Expr, SolPath, File, Variant, Item, ItemEvent, ItemError, Modifier};

pub enum FoundNode {
    // Contracts
    ContractDefName(ItemContract),
    ContractDefInheritance(ItemContract, Modifier),
    ContractInstantiation(ItemContract, Option<ItemFunction>, ExprNew),


    // Functions
    FunctionDefName(ItemContract, ItemFunction),
    FunctionDefParameterName(ItemContract, ItemFunction, VariableDeclaration, SolIdent),

    // Properties
    PropertyDefName(ItemContract, VariableDefinition, SolIdent),
    //PropertyDefType(ItemContract, VariableDefinition, Type),
    ConstantVariableDefName(VariableDefinition, SolIdent),









    //ContractScope(ItemContract, Option<ItemFunction>, SolPath),

    //FunctionDefParameterType(ItemContract, ItemFunction, VariableDeclaration, Type),

    FunctionUsageName(ItemContract, ItemFunction, ExprCall),

    // Properties

    // Variables
    VariableDefName(Option<ItemContract>, Option<ItemFunction> VarDeclDecl, VariableDeclaration, Option<SolIdent>),
    VariableDefType(File,ItemContract, VarDeclDecl, VariableDeclaration, Type),
    VariableUsageName(Option<ItemContract>, Option<ItemFunction>, Expr, SolIdent),

    // Enum
    EnumDefName(File,Option<ItemContract>, ItemEnum, SolIdent),
    EnumDefValue(Option<ItemContract>, ItemEnum, Variant, SolIdent),


    // Structs
    StructDefName(Option<ItemContract>, SolIdent),
    StructInstantiation(ItemContract, Option<ItemFunction>, SolIdent),
    StructDefPropertyName(ItemContract, Option<ItemFunction>, VariableDeclaration, SolIdent),
    //StructUsageName(ItemContract, Option<ItemFunction>, Expr, Type),

    //StructUsageProperty(ItemContract, Option<ItemFunction>, Expr, SolIdent, Type),


    // Errors
    ErrorDefName(Option<ItemContract>, ItemError, SolIdent),
    ErrorDefParameter(Option<ItemContract>, ItemError, VariableDeclaration),
    //ErrorUsageName(ItemContract, Option<ItemFunction>, ExprCall, SolIdent),


    // Events
    EventDefName(ItemContract, ItemEvent, SolIdent),
    EventDefParameter(ItemContract, ItemEvent, VariableDeclaration),
    //EventUsageName(ItemContract, Option<ItemFunction>, Expr, SolIdent),

    TypeUsage(Option<ItemContract>, Option<ItemFunction>, Option<Expr>, Type),

    //TODO type cast
    // TODO super ast node
}