use std::io::{BufReader, BufWriter};
use ::dap::server::Server;
use ::dap::requests::Command;
use ::dap::responses::ResponseBody;

use thiserror::Error;

mod dap;

#[derive(Error, Debug)]
enum MyAdapterError {
  #[error("Unhandled command")]
  UnhandledCommandError,

  #[error("Missing command")]
  MissingCommandError,
}

type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> DynResult<()> {
  let output = BufWriter::new(std::io::stdout());
  let input = BufReader::new(std::io::stdin());
  let mut server = Server::new(input, output);

  // TODO handle launch command once we know what to send
  let req = match server.poll_request()? {
    Some(req) => req,
    None => return Err(Box::new(MyAdapterError::MissingCommandError)),
  };
  if let Command::Initialize(_) = req.command {
    let rsp = req.success(ResponseBody::Initialize(Default::default()));

    server.respond(rsp)?;
    let _ = dap::run_session(server);
    Ok(())
  } else {
    Err(Box::new(MyAdapterError::UnhandledCommandError))
  }
}
