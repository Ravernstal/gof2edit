use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum UnpackTarget {
    Lang,
    Ships,
    Stations,
    Systems,
}
