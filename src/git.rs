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

use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, io};

use colored::*;
use regex::Regex;

use crate::i18n::Localization;

pub struct GitBroom {
    repository: Option<String>,
    branch: Option<String>,
    dry_run: bool,
    current_dir: Option<PathBuf>,
    localization: Localization,
}

struct Branch {
    name: String,
    protected: bool,
}

impl GitBroom {
    pub fn new(repository: Option<String>, branch: Option<String>, dry_run: bool) -> Self {
        Self {
            repository,
            branch,
            dry_run,
            current_dir: {
                match env::current_dir() {
                    Ok(path) => Some(path),
                    Err(_) => None,
                }
            },
            localization: Localization::new(),
        }
    }

    pub fn check_git() -> Result<(), io::Error> {
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

            if !protected_branches.is_empty() {
                println!(
                    "{}",
                    self.localization.get_message_with_count_and_one_arg(
                        "found-merged-protected",
                        protected_branches.len(),
                        String::from("branch"),
                        branch.bold().underline().to_string()
                    )
                );

                for branch in &protected_branches {
                    println!("  * {}", branch.blue());
                }

                println!(
                    "{}",
                    self.localization.get_message_with_count(
                        "branches-wont-be-deleted",
                        protected_branches.len()
                    )
                );

                if !not_protected_branches.is_empty() {
                    println!();
                }
            }

            if !not_protected_branches.is_empty() {
                println!(
                    "{}",
                    self.localization.get_message_with_count_and_one_arg(
                        "found-merged",
                        not_protected_branches.len(),
                        String::from("branch"),
                        branch.bold().underline().to_string(),
                    )
                );

                for branch in &not_protected_branches {
                    println!("  * {}", branch.green());
                }

                if !self.dry_run {
                    match self.read_user_input(
                        self.localization.get_message("delete-selection") + " ",
                        'n',
                    )? {
                        'a' => self.delete_all_branches(not_protected_branches)?,
                        's' => self.ask_delete_all_branches(not_protected_branches)?,
                        _ => println!("{}", self.localization.get_message("no-branch-deleted"),),
                    }
                }
            }
        } else {
            println!(
                "{}",
                self.localization.get_message_with_one_arg(
                    "no-merged-branch",
                    String::from("branch"),
                    branch.bold().to_string(),
                )
            );
        }

        Ok(())
    }

    fn delete_all_branches(&self, branches: Vec<String>) -> Result<(), io::Error> {
        println!();
        for branch in &branches {
            if self.delete_branch(branch)? {
                println!(
                    "{}",
                    self.localization.get_message_with_one_arg(
                        "branch-deleted",
                        String::from("branch"),
                        branch.bold().to_string(),
                    )
                );
            } else {
                println!(
                    "{}",
                    self.localization.get_message_with_one_arg(
                        "branch-cannot-be-deleted",
                        String::from("branch"),
                        branch.bold().to_string(),
                    )
                );
            }
        }

        Ok(())
    }

    fn ask_delete_all_branches(&self, branches: Vec<String>) -> Result<(), io::Error> {
        println!();
        for branch in &branches {
            match self.read_user_input(
                self.localization.get_message_with_one_arg(
                    "delete-branch-yes-no",
                    String::from("branch"),
                    branch.bold().to_string(),
                ) + " ",
                'n',
            )? {
                'y' => {
                    if self.delete_branch(branch)? {
                        println!(
                            "{}",
                            self.localization.get_message_with_one_arg(
                                "branch-deleted",
                                String::from("branch"),
                                branch.bold().to_string(),
                            )
                        );
                    } else {
                        println!(
                            "{}",
                            self.localization.get_message_with_one_arg(
                                "branch-cannot-be-deleted",
                                String::from("branch"),
                                branch.bold().to_string(),
                            )
                        );
                    }
                }
                _ => println!(
                    "{}",
                    self.localization.get_message_with_one_arg(
                        "branch-has-not-been-deleted",
                        String::from("branch"),
                        branch.bold().to_string(),
                    )
                ),
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
            return Err(io::Error::new(
                ErrorKind::Other,
                self.localization.get_message("no-valid-branch-found"),
            ));
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

    fn is_protected_branch(&self, branch: &String, protected_branches: &Vec<Regex>) -> bool {
        for protected_branch in protected_branches.iter() {
            if protected_branch.is_match(&branch) {
                return true;
            }
        }

        false
    }

    fn get_merged_branches(&self, branch: &String) -> Result<Vec<Branch>, io::Error> {
        let protected_branches = self.get_protected_branches();

        let output = Command::new("git")
            .arg("branch")
            .arg("--merged")
            .arg(branch)
            .output()?;

        let mut branches: Vec<Branch> = Vec::new();

        String::from_utf8_lossy(&output.stdout)
            .lines()
            .for_each(|line| {
                let line = line.trim().to_string();
                if !line.starts_with('*') && !line.eq(branch) {
                    let branch = Branch {
                        name: String::from(&line),
                        protected: self.is_protected_branch(&line, &protected_branches),
                    };

                    branches.push(branch);
                }
            });

        Ok(branches)
    }

    fn get_protected_branches(&self) -> Vec<Regex> {
        match &mut env::current_dir() {
            Ok(path) => {
                path.push(".git");
                match gix_config::File::from_git_dir(path.clone()) {
                    Ok(file) => match file.string_by_key("broom.protectedbranches") {
                        Some(branches) => branches
                            .to_string()
                            .split(",")
                            .map(String::from)
                            .filter_map(|re| Regex::new(&re).ok())
                            .collect(),
                        None => Vec::new(),
                    },
                    Err(_) => Vec::new(),
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
}
