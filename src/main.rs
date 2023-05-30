mod git;

use clap::Parser;

/// Helper tool to clean up local merged Git branches.
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

fn main() {
    let args = Args::parse();

    match git::GitBroom::check_git() {
        Ok(_) => git::GitBroom::new(args.repository, args.branch, args.quiet).broom(),
        Err(e) => println!("{e}"),
    }
}
