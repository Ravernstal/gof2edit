use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum BinaryVersion {
    /// libgof2hdaa.so
    Android,
    /// Galaxy_on_Fire_2_HD
    Ios,
}
