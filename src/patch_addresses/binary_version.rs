use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum BinaryVersion {
    /// KiritoJPK's full HD Android mod
    AndroidKiritoJpk,
    /// The stock iOS binary
    Ios,
}
