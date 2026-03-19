use clap::Parser;
use emmylua_ls::cmd_args::CmdArgs;
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let cmd_args = CmdArgs::parse();
    emmylua_ls::run_ls(cmd_args).await
}
