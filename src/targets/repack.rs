use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum RepackTarget {
    Lang,
    Ships,
    Stations,
    Systems,
}
