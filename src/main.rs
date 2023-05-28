use clap::Parser;
use std::process::Command;

/// Helper tool to clean up local merged Git branches.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of Git repository. Current path if not specified.
    repository: Option<String>,

    /// Branch to check if local branches are merged on.
    #[arg(short, long)]
    branch: Option<String>,
}

fn print_args(args: &Args) {
    let branch = match args.branch.as_deref() {
        None => String::from("current"),
        Some(branch) => String::from(branch),
    };
    println!("Git branch: {}", branch);

    let repository = match args.repository.as_deref() {
        None => String::from("current"),
        Some(repository) => String::from(repository),
    };
	println!("Git repository: {}", repository);
}

fn check_git() -> bool {
	let output = Command::new("git").arg("--version").output().expect("Fail to execute git --version.");

    return if !output.status.success() {
        println!("No git found: {}", String::from_utf8_lossy(&output.stderr));
        false
    } else {
        println!("Using {}", String::from_utf8_lossy(&output.stdout));
        true
    }
}

fn main() {
    let args = Args::parse();

	#[cfg(debug_assertions)]
	print_args(&args);

	if check_git() {

	}
}
