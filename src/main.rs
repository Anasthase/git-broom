mod git;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    repository: Option<String>,
    /// Name of the person to greet
    #[arg(short, long)]
    branch: Option<String>,
}

fn main() {
    let args = Args::parse();
    dbg!(&args);

    match git::GitBroom::check_git() {
        Ok(_) => git::GitBroom::new(args.repository, args.branch).broom(),
        Err(e) => println!("{e}"),
    }
}
