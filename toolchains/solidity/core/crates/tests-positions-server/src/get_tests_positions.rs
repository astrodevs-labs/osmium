use osmium_libs_lsp_server_wrapper::lsp_types::{request::Request, Range};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetTestsPositionsParams {
    pub file_content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TestContract {
    pub contract_range: Range,
    pub tests_ranges: Vec<Range>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetTestsPositionsResponse {
    pub ranges: Vec<TestContract>,
}

pub struct GetTestsPositionsRequest {}

impl Request for GetTestsPositionsRequest {
    type Params = GetTestsPositionsParams;
    type Result = GetTestsPositionsResponse;
    const METHOD: &'static str = "osmium/getTestsPositions";
}
