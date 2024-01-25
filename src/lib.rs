pub mod ips;

pub type SubcommandError = Box<dyn std::error::Error>;
type SubcommandResult = std::result::Result<(), SubcommandError>;

#[derive(fcla::FromArgs)]
pub enum Subcommand {
    Ips { subcommand: ips::Ips },
}

pub fn main(subcommand: Subcommand) -> SubcommandResult {
    use Subcommand::*;

    match subcommand {
        Ips { subcommand } => ips::main(subcommand),
    }
}

mod prelude {
    pub(crate) use super::SubcommandResult;
    pub use fcla::FromArgs;
    pub use std::{error::Error, fmt, fs::File, io, io::prelude::*, path::Path};
}
