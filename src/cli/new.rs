use anyhow::{bail, Result};

use super::NewArgs;

pub fn run(args: NewArgs) -> Result<()> {
    let _ = args;
    bail!("`kb-claude new` is not implemented yet")
}
