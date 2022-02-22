mod parser;
mod runner;

use anyhow::bail;
use anyhow::Error;
pub use runner::run;

pub type Result<T> = std::result::Result<T, Error>;
