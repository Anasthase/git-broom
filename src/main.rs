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

use std::error;

use clap::Parser;

mod git;
mod i18n;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of Git repository. Current path if not specified.
    repository: Option<String>,
    /// Branch to check if local branches are merged on.
    #[arg(short, long)]
    branch: Option<String>,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();

    git::GitBroom::check_git()?;
    git::GitBroom::new(args.repository, args.branch).broom()?;

    Ok(())
}
