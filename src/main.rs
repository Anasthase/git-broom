mod git;

use std::error;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of Git repository. Current path if not specified.
    repository: Option<String>,
    /// Branch to check if local branches are merged on.
    #[arg(short, long)]
    branch: Option<String>,
    /// Only print required information.
    #[arg(short, long)]
    quiet: bool,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();

    git::GitBroom::check_git()?;
    git::GitBroom::new(args.repository, args.branch, args.quiet).broom()?;

    Ok(())
}
