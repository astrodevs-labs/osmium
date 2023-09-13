use proc_macro2::{LexError, TokenStream};
/**
 * extract.rs
 * Extract AST from solidity source code
 * author: 0xMemoryGrinder
*/
use std::str::FromStr;
use syn::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExtractError {
    #[error("Tokenization error: {0}")]
    Tokenize(#[from] LexError),
    #[error("Parsing error")]
    Parse(#[from] Error),
}

pub fn extract_ast_from(source: String) -> Result<syn_solidity::File, ExtractError> {
    let tokens = TokenStream::from_str(source.as_str())?;
    let ast = syn_solidity::parse2(tokens)?;
    Ok(ast)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn test_extract_ast_from_good() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests");
        path.push("files");
        path.push("good.sol");
        let source = fs::read_to_string(path).unwrap();
        let res = extract_ast_from(source);
        assert!(res.is_ok());
    }

    #[test]
    fn test_extract_ast_from_invalid_token() {
        let source = String::from("contract test { function test() public | uint a = 1 } }");
        let result = extract_ast_from(source);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Tokenization error: cannot parse string into token stream"
        );
    }

    #[test]
    fn test_extract_ast_from_missing_semicolumn() {
        let source = String::from("contract test { function test() public { uint a = 1 } }");
        let result = extract_ast_from(source);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Parsing error");
    }
}
