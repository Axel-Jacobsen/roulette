use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Commands to run (any of grep, mypy, ruff, flake8) - defaults to all of them
    #[arg(short, long, num_args = 0..)]
    pub commands: Option<Vec<String>>,
    /// Optional keywords for grep: defaults to "TODO" and "FIXME"
    #[arg(short, long, num_args = 0..)]
    pub grep_keywords: Option<Vec<String>>,
    /// Print supported commands
    #[arg(long, action=clap::ArgAction::SetTrue)]
    pub supported: bool
}
