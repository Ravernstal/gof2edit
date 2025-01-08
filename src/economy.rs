use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Economy {
    /// The original game economy
    Original,
    /// The economy on Android versions of the game
    Android,
}
