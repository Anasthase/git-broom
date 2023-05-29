use std::{env, io};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
pub struct GitBroom {
    repository: Option<String>,
    branch: Option<String>,
    current_dir: Option<PathBuf>,
}

impl GitBroom {
    pub fn new(repository: Option<String>, branch: Option<String>) -> Self {
        Self {
            repository,
            branch,
            current_dir: {
                match env::current_dir() {
                    Ok(path) => Some(path),
                    Err(_) => None,
                }
            }
        }
    }

    pub fn check_git() ->  Result<(), String> {
        let output = Command::new("git").arg("status").output();

        match output {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Unable to found Git: {}", e.to_string())),
        }
    }

    pub fn broom(&self) {
        if let Some(repository) = &self.repository {
            let working_dir = Path::new(repository);
            env::set_current_dir(working_dir).expect(format!("Unable to set working dir to {}.", &repository).as_str());
        }

        if let Some(branch) = self.get_working_branch() {
            let merged_branches = self.get_merged_branches(&branch);
            self.process_branches(branch, merged_branches);
        }

        if let Some(path) = &self.current_dir {
            env::set_current_dir(path).expect("Unable to set working dir to initial path.");
        }
    }

    fn process_branches(&self, branch: String, merged_branches: Vec<String>) {
        if merged_branches.len() > 0 {
            println!("Found {} merged branches on {}:", merged_branches.len(), branch);

            for branch in &merged_branches {
                println!("  - {branch}");
            }

            print!("Delete [a]ll, [s]elected, [c]cancel: ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();

            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read choice.");

            if !choice.is_empty() && choice.trim().len() == 1 {
                let ch = choice.to_lowercase().chars().next().unwrap();
                match ch {
                    'a' => self.delete_all_branches(merged_branches),
                    's' => self.ask_delete_all_branches(merged_branches),
                    _ => println!("No branch deleted."),
                }
            } else {
                println!("No branch deleted.");
            }
        } else {
            println!("No merged branches found on {}.", branch);
        }
    }

    fn delete_all_branches(&self, branches: Vec<String>) {
        println!("---");
        for branch in &branches {
            if self.delete_branch(branch) {
                println!("Branch {branch} deleted.");
            } else {
                println!("{branch} has not been deleted.");
            }
        }
    }

    fn ask_delete_all_branches(&self, branches: Vec<String>) {
        println!("---");
        for branch in &branches {
            print!("Delete branch \"{branch}\"? [y]es, [n]o: ");
            io::stdout().flush().unwrap();

            let mut choice = String::new();

            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read choice.");

            let ch = choice.to_lowercase().chars().next().unwrap();
            match ch {
                'y' => {
                    if self.delete_branch(branch) {
                        println!("Branch {branch} deleted.");
                    } else {
                        println!("{branch} has not been deleted.");
                    }
                }
                _ => println!("{branch} has not been deleted."),
            }
        }
    }

    fn delete_branch(&self, branch: &String) -> bool {
        let output = Command::new("git")
            .arg("branch")
            .arg("-d")
            .arg(branch)
            .output()
            .expect("Unable to delete branch.");

        output.status.success()
    }

    fn get_working_branch(&self) -> Option<String> {
        match &self.branch {
            None => self.get_current_branch(),
            Some(branch) => Some(branch.to_string()),
        }
    }

    fn get_current_branch(&self) -> Option<String> {
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output()
            .expect("Unable to get current branch.");

        if output.status.success() {
            Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            println!("{}", String::from_utf8_lossy(&output.stderr));
            None
        }
    }

    fn get_merged_branches(&self, branch: &String) -> Vec<String> {
        let output = Command::new("git")
            .arg("branch")
            .arg("--merged")
            .arg(branch)
            .output()
            .expect("Unable to get merged branches on {branch}.");

        let mut branches: Vec<String> = Vec::new();

        String::from_utf8_lossy(&output.stdout).lines().for_each(|line| {
            let line = line.trim();
            if !line.starts_with('*') && !line.eq(branch) {
                branches.push(String::from(line));
            }
        });

        branches
    }
}
