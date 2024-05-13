use crate::CmdExecutor;
use clap::Parser;
use zxcvbn::zxcvbn;

#[derive(Debug, Parser)]
pub struct PasswdOpt {
    #[arg(long, default_value_t = 12)]
    pub len: u8,
    #[arg(long, default_value_t = false)]
    pub no_upper: bool,
    #[arg(long, default_value_t = false)]
    pub no_lower: bool,
    #[arg(long, default_value_t = false)]
    pub no_number: bool,
    #[arg(long, default_value_t = false)]
    pub no_symbol: bool,
}

impl CmdExecutor for PasswdOpt {
    async fn exec(self) -> anyhow::Result<()> {
        let passwd = crate::process_passwd_gen(
            self.len,
            self.no_upper,
            self.no_lower,
            self.no_number,
            self.no_symbol,
        )?;

        println!("{}", passwd);

        let estimate = zxcvbn(&passwd, &[])?;
        eprintln!("Password strength: {}", estimate.score());

        Ok(())
    }
}
