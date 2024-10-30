use std::{error::Error, io, path::PathBuf};

use ashen::asset::pack_file::{directory::VirtualFileSystem, PackFile};
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
pub enum Cli {
    /// Output packfile contents.
    Output {
        /// The path of the 'pack'.file
        path: PathBuf,
    },
}

// enum AssetKind {
//     Copyright,
// }

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match cli {
        Cli::Output { path } => {
            let pack = std::fs::read(path)?;
            let (_, pack) = PackFile::new(&pack)?;

            let dir = VirtualFileSystem::from_106_packfile(pack)
                .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))?;

            let handle = dir.read("/copyright")?;

            std::fs::write("./output/cli/copyright.txt", handle.bytes())?;
        }
    }

    Ok(())
}
