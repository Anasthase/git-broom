use std::{env, io};
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
pub struct GitBroom {
    repository: Option<String>,
    branch: Option<String>,
    quiet: bool,
    current_dir: Option<PathBuf>,
}

impl GitBroom {
    pub fn new(repository: Option<String>, branch: Option<String>, quiet: bool) -> Self {
        Self {
            repository,
            branch,
            quiet,
            current_dir: {
                match env::current_dir() {
                    Ok(path) => Some(path),
                    Err(_) => None,
                }
            }
        }
    }

    pub fn check_git() ->  Result<(), io::Error> {
        Command::new("git").arg("--version").output()?;
        Command::new("git").arg("status").output()?;

        Ok(())
    }

    pub fn broom(&self) -> Result<(), io::Error> {
        if let Some(repository) = &self.repository {
            env::set_current_dir(Path::new(repository))?;
        }

        self.broom_branch(self.get_working_branch()?)?;

        if let Some(path) = &self.current_dir {
            env::set_current_dir(path)?;
        }

        Ok(())
    }

    fn broom_branch(&self, branch: String) -> Result<(), io::Error> {
        let merged_branches= self.get_merged_branches(&branch)?;

        if merged_branches.len() > 0 {
            self.print_conditional_message(format!("Found {} merged branches on {}:", merged_branches.len(), branch));

            for branch in &merged_branches {
                println!("  - {branch}");
            }

            match self.read_user_input(String::from("Delete [a]ll, [s]elected, [n]one: "), 'n')? {
                'a' => self.delete_all_branches(merged_branches)?,
                's' => self.ask_delete_all_branches(merged_branches)?,
                _ => self.print_conditional_message(format!("No branch deleted.")),
            }
        } else {
            self.print_conditional_message(format!("No merged branches found on {}.", branch));
        }

        Ok(())
    }

    fn delete_all_branches(&self, branches: Vec<String>) -> Result<(), io::Error> {
        self.print_conditional_message("---".to_string());
        for branch in &branches {
            if self.delete_branch(branch)? {
                self.print_conditional_message(format!("Branch {branch} deleted."));
            } else {
                self.print_conditional_message(format!("{branch} has not been deleted."));
            }
        }

        Ok(())
    }

    fn ask_delete_all_branches(&self, branches: Vec<String>) -> Result<(), io::Error> {
        self.print_conditional_message("---".to_string());
        for branch in &branches {
            match self.read_user_input(format!("Delete branch \"{branch}\"? [y]es, [n]o: "), 'n')? {
                'y' => {
                    if self.delete_branch(branch)? {
                        self.print_conditional_message(format!("Branch {branch} deleted."));
                    } else {
                        self.print_conditional_message(format!("{branch} has not been deleted."));
                    }
                }
                _ => self.print_conditional_message(format!("{branch} has not been deleted.")),
            }
        }

        Ok(())
    }

    fn delete_branch(&self, branch: &String) -> Result<bool, io::Error> {
        let output = Command::new("git")
            .arg("branch")
            .arg("-d")
            .arg(branch)
            .output()?;

        Ok(output.status.success())
    }

    fn get_working_branch(&self) -> Result<String, io::Error> {
        let working_branch = match &self.branch {
            None => self.get_current_branch(),
            Some(branch) => Ok(branch.trim().to_string()),
        }?;

        if working_branch.is_empty() {
            return Err(io::Error::new(ErrorKind::Other, "No valid branch found. Is the repository a valid Git repository?"));
        }

        Ok(working_branch)
    }

    fn get_current_branch(&self) -> Result<String, io::Error> {
        let output = Command::new("git")
            .arg("rev-parse")
            .arg("--abbrev-ref")
            .arg("HEAD")
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    fn get_merged_branches(&self, branch: &String) -> Result<Vec<String>, io::Error> {
        let protected_branches = self.get_protected_branches();

        let output = Command::new("git")
            .arg("branch")
            .arg("--merged")
            .arg(branch)
            .output()?;

        let mut branches: Vec<String> = Vec::new();

        String::from_utf8_lossy(&output.stdout).lines().for_each(|line| {
            let line = line.trim().to_string();
            if !line.starts_with('*') && !line.eq(branch) && !protected_branches.contains(&line) {
                branches.push(String::from(line));
            }
        });

        Ok(branches)
    }

    fn get_protected_branches(&self) -> Vec<String> {
        match &mut env::current_dir() {
            Ok(path) => {
                path.push(".git");
                match gix_config::File::from_git_dir(&path) {
                    Ok(file) => {
                        match file.string_by_key("broom.protectedbranches") {
                            Some(branches) => branches.to_string().split(",").map(String::from).collect(),
                            None => Vec::new(),
                        }
                    },
                    Err(_)  => Vec::new(),
                }
            }
            Err(_) => Vec::new(),
        }
    }

    fn read_user_input(&self, message: String, default: char) -> Result<char, io::Error> {
        print!("{}", message);
        io::stdout().flush().unwrap();

        let mut choice = String::new();

        io::stdin().read_line(&mut choice)?;

        if !choice.is_empty() && choice.trim().len() == 1 {
            Ok(choice.to_lowercase().chars().next().unwrap())
        } else {
            Ok(default)
        }
    }

    fn print_conditional_message(&self, message: String) {
        if !self.quiet {
            println!("{message}");
        }
    }
}
