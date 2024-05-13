use clap::Parser;
use enum_dispatch::enum_dispatch;

pub use self::{base64::*, csv::*, passwd::*};

mod base64;
mod csv;
mod passwd;

#[derive(Debug, Parser)]
#[command(name = "jcli", version = "0.0.1", author = "jrmarcco", about, long_about = None)]
pub struct Jcli {
    #[command(subcommand)]
    pub cmd: SubCmd,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum SubCmd {
    #[command(name = "csv", about = "Show csv or convert csv to other formats.")]
    Csv(CsvOpts),
    #[command(name = "passwd", about = "Generate a random password.")]
    Passwd(PasswdOpt),
    #[command(subcommand, about = "Base64 encode/decode.")]
    Base64(Base64Opt),
}
