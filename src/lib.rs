mod cli;
mod process;

pub use cli::*;
use enum_dispatch::enum_dispatch;
pub use process::*;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn exec(self) -> anyhow::Result<()>;
}
