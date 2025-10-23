use anyhow::{bail, Result};

use super::LinkArgs;

pub fn run(args: LinkArgs) -> Result<()> {
    let _ = args;
    bail!("`kb-claude link` is not implemented yet")
}
