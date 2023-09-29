/**
 * find_node.rs
 * Enum for all possible ast types that can be found
 * author: 0xMemoryGrinder
 */

use syn_solidity::{ItemContract, ItemEnum};

pub enum FoundNode {
    // Contract Definition
    ContractDef(ItemContract),
    ContractRef(ItemContract),





    FileEnum(ItemEnum),
    ContractEnum((ItemContract, ItemEnum))

}