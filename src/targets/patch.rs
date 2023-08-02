use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum PatchTarget {
    /// stations.bin
    Stations,
    /// systems.bin
    Systems,
}
