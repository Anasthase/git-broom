/*
Git Broom
Copyright (C) 2023  All contributors.

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

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

struct Branch {
    name: String,
    protected: bool,
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
        let merged_branches = self.get_merged_branches(&branch)?;

        if !merged_branches.is_empty() {

            let protected_branches: Vec<String> = merged_branches
                .iter()
                .filter(|branch| branch.protected)
                .map(|branch| String::from(&branch.name))
                .collect();

            let not_protected_branches: Vec<String> = merged_branches
                .iter()
                .filter(|branch| !branch.protected)
                .map(|branch| String::from(&branch.name))
                .collect();

            if !self.quiet && !protected_branches.is_empty() {
                println!("Found {} merged but protected branches on {}:", protected_branches.len(), branch);

                for branch in &protected_branches {
                    println!("  - {branch}");
                }

                println!("These branches will not be deleted.");

                if !not_protected_branches.is_empty() {
                    println!("---");
                }
            }

            if !not_protected_branches.is_empty() {
                self.print_conditional_message(format!("Found {} merged branches on {}:", not_protected_branches.len(), branch));

                for branch in &not_protected_branches {
                    println!("  - {branch}");
                }

                match self.read_user_input(String::from("Delete [a]ll, [s]elected, [n]one: "), 'n')? {
                    'a' => self.delete_all_branches(not_protected_branches)?,
                    's' => self.ask_delete_all_branches(not_protected_branches)?,
                    _ => self.print_conditional_message(format!("No branch deleted.")),
                }
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

    fn get_merged_branches(&self, branch: &String) -> Result<Vec<Branch>, io::Error> {
        let protected_branches = self.get_protected_branches();

        let output = Command::new("git")
            .arg("branch")
            .arg("--merged")
            .arg(branch)
            .output()?;

        let mut branches: Vec<Branch> = Vec::new();

        String::from_utf8_lossy(&output.stdout).lines().for_each(|line| {
            let line = line.trim().to_string();
            if !line.starts_with('*') && !line.eq(branch) {
                let branch = Branch {
                    name: String::from(&line),
                    protected: protected_branches.contains(&line),
                };

                branches.push(branch);
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
