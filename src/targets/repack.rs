use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum RepackTarget {
    /// names_*.bin
    Lang,
    /// ships.bin
    Ships,
    /// stations.bin
    Stations,
    /// systems.bin
    Systems,
}
