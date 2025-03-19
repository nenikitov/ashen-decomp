use std::{
    borrow::Cow,
    env, fs, io,
    path::{Path, PathBuf},
};

use ashen::asset::pack_file::{PackFile, directory::VirtualFileSystem};
use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about)]
pub enum Cli {
    // TODO(Unavailable): Unpack Command?
    /// Output packfile contents.
    Output {
        /// The kind of files to output
        kind: Vec<ParsableAsset>,
        /// The output directory
        #[arg(default_value = "output")]
        out: PathBuf,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum ParsableAsset {
    Copyright,
    GammaTable,
    ColorMap,
    SkyBox,
    Model,
    StringTable,
}

impl ParsableAsset {
    pub fn into_path(self) -> &'static Path {
        match self {
            Self::Copyright => Path::new("/copyright"),
            Self::GammaTable => Path::new("/gamma_table"),
            Self::ColorMap => Path::new("/color_map"),
            Self::SkyBox => Path::new("/sky_box"),
            Self::Model => Path::new("/model"),
            Self::StringTable => Path::new("/string_table"),
        }
    }
}

fn main() -> eyre::Result<()> {
    let cli = Cli::parse();

    let packfile_path = env::var_os("ASHEN_CLI_PACKFILE");
    let packfile_path = match packfile_path {
        Some(path) => Cow::Owned(PathBuf::from(path)),
        None => {
            eprintln!("Defaulting ASHEN_CLI_PACK_FILE to \"packfile.dat\"");
            Cow::Borrowed(Path::new("packfile.dat"))
        }
    };

    let packfile = fs::read(packfile_path)?;
    let packfile = PackFile::new(&packfile)?.1;

    match cli {
        Cli::Output { kind, out } => {
            let dir = VirtualFileSystem::from_106_packfile(packfile)
                .ok_or(io::Error::from(io::ErrorKind::InvalidInput))?;

            for kind in kind {
                match kind {
                    ParsableAsset::Copyright => {
                        let path = kind.into_path();
                        let file = dir.read(path)?;
                        fs::write(out.join(path), file.bytes())?;
                    }
                    ParsableAsset::GammaTable => {
                        let path = kind.into_path();
                        let _file = dir.read(path)?;
                    }
                    ParsableAsset::ColorMap => todo!(),
                    ParsableAsset::SkyBox => todo!(),
                    ParsableAsset::Model => todo!(),
                    ParsableAsset::StringTable => todo!(),
                }
            }

            let handle = dir.read("/copyright")?;

            fs::write("./output/cli/copyright.txt", handle.bytes())?;
        }
    }

    Ok(())
}
