use clap::Parser;

use jcli::{CmdExecutor, Jcli};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let jcli = Jcli::parse();
    jcli.cmd.exec().await?;

    Ok(())
}
