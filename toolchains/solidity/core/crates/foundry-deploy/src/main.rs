use std::process::Command;
use tower_lsp::{Client, LanguageServer};

#[derive(Debug)]
struct Backend {
    client: Client,
}

impl<InitializeResult> LanguageServer for Backend {
    fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {}

    fn deploy_contract() {
        //get infos thanks to the client
        let contractName = "Test";
        let rpc_url = "rpc_urlTest";
        let key = "keyTest";

        // Create a new instance of the command
        let mut cmd = Command::new("forge");
        // Add arguments to the command
        cmd.arg("create");
        cmd.arg("--rpc-url");
        cmd.arg(rpc_url);
        cmd.arg("--private-key");
        cmd.arg(key);
        cmd.arg("--verify");
        cmd.arg(contractName);

        // Execute the command and capture the result
        let result = cmd.status();

        // Handle the result
        match result {
            Ok(status) => {
                if status.success() {
                    println!("Command executed successfully!");
                } else {
                    println!("Command failed with exit code: {:?}", status.code());
                }
            }
            Err(e) => {
                println!("Error executing the command: {:?}", e);
            }
        }
    }

    fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

async fn main() {
}