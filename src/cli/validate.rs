use anyhow::{bail, Result};

use super::ValidateArgs;

pub fn run(args: ValidateArgs) -> Result<()> {
    let _ = args;
    bail!("`kb-claude validate` is not implemented yet")
}
