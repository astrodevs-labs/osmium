use osmium_libs_lsp_server_wrapper::lsp_types::{
    Url,
    request::Request,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ContentRequestParams {
    pub uri: String,
}

pub struct ContentRequest {
}

#[derive(Serialize, Deserialize)]
pub struct ContentResponse {
    pub content: String,
}

impl Request for ContentRequest {
    type Params = ContentRequestParams;
    type Result = ContentResponse;
    const METHOD: &'static str = "osmium/getContent";
}
