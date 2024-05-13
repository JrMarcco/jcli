use enum_dispatch::enum_dispatch;

pub use cli::*;
pub use process::*;

mod cli;
mod process;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn exec(self) -> anyhow::Result<()>;
}
