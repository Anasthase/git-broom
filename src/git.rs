use std::env;
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
            let merged_branches = self.get_merged_branches(branch);

            for branch in &merged_branches {
                println!("{branch}");
            }
        }

        if let Some(path) = &self.current_dir {
            env::set_current_dir(path).expect("Unable to set working dir to initial path.");
        }
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

    fn get_merged_branches(&self, branch: String) -> Vec<String> {
        let output = Command::new("git")
            .arg("branch")
            .arg("--merged")
            .arg(branch)
            .output()
            .expect("Unable to get merged branches on {branch}.");

        let mut branches: Vec<String> = Vec::new();

        String::from_utf8_lossy(&output.stdout).lines().for_each(|line| {
            if !line.starts_with('*') {
                branches.push(String::from(line.trim()));
            }
        });

        branches
    }
}
