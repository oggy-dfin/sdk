use crate::lib::environment::Environment;
use crate::lib::error::{DfxError, DfxResult};

use clap::Parser;

/// Forces unpacking the cache from this dfx version.
#[derive(Parser)]
#[clap(name("install"))]
pub struct CacheInstall {}

pub fn exec(env: &dyn Environment, _opts: CacheInstall) -> DfxResult {
    env.get_cache().force_install().map_err(DfxError::from)
}
