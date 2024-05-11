mod csv;

use clap::Parser;
use enum_dispatch::enum_dispatch;

pub use self::csv::*;

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
}
