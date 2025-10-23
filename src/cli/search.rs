use anyhow::{bail, Result};

use super::SearchArgs;

pub fn run(args: SearchArgs) -> Result<()> {
    let _ = args;
    bail!("`kb-claude search` is not implemented yet")
}
