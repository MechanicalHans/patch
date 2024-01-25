// https://fileformats.archiveteam.org/wiki/IPS_(binary_patch_format)

mod apply;

use crate::prelude::*;

#[derive(FromArgs)]
pub enum Ips {
    Apply { patch: Box<Path>, target: Box<Path> },
}

pub fn main(args: Ips) -> SubcommandResult {
    use Ips::*;

    match args {
        Apply { patch, target } => apply::main(patch, target)?,
    }
    Ok(())
}
