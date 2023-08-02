use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum PatchTarget {
    Stations,
    Systems,
}
