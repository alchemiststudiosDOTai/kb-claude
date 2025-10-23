use anyhow::{bail, Result};

use super::ManifestArgs;

pub fn run(args: ManifestArgs) -> Result<()> {
    let _ = args;
    bail!("`kb-claude manifest` is not implemented yet")
}
