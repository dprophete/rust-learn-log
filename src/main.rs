pub mod log;

use anyhow::Result;
use clap::Parser;
use log::{llog, lloge, log, loge};

#[derive(clap::Parser, Debug)]
#[command(about = "A basic tool to log", long_about = None)]
pub struct MainArgs {}

fn main() -> Result<()> {
    let _args = MainArgs::parse();
    log("Hello, dda");
    loge("an error");
    llog("short section", "a message");
    llog("a pretty long section name", "a message");
    lloge("section", "an error");
    Ok(())
}
