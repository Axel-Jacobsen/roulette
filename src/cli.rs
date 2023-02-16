use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to directory (defaults to `.`). Note that `clippy` and `git grep` don't take a path,
    /// and require you to run `roulette` where you want the commands to be executed.
    #[arg(short, long)]
    pub path: Option<PathBuf>,

    /// Commands to run (any of git_grep, rip_grep, grep, mypy, ruff, flake8) - defaults git grep and mypy.
    #[arg(short, long, num_args = 1..)]
    pub commands: Option<Vec<String>>,

    /// Optional keywords for grep: defaults to "TODO" and "FIXME".
    #[arg(short, long, num_args = 1..)]
    pub grep_keywords: Option<Vec<String>>,

    /// Print supported commands
    #[arg(long, action=clap::ArgAction::SetTrue)]
    pub supported: bool,

    /// Print out failed commands (instead of silently ignoring, good for debugging)
    #[arg(long, action=clap::ArgAction::SetTrue)]
    pub debug: bool,

    /// Print out every result, useful for debugging
    #[arg(long, action=clap::ArgAction::SetTrue)]
    pub all: bool,
}
