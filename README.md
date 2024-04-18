# Git Broom

Git Broom is a command line helper tool to clean up local merged Git branches.

Common workflows around Git usually involve the creation of branches that are later deleted from the reference repository, while still remaining on developers' repositories despite being useless.

Listing such branches is easy, using the `git branch --merged` command, but then one will have to either parse and pipe this output to another Git command, or manually type the Git command to delete the branches.

Git Broom simply display the merged branches, and propose to delete them for you, either all of them or selected one.

## Disclaimer

Git Broom deletes branches in a safe way (similar to  the `git branch -d` command). However, use it at your own risk. See §15 and §16 of the [GPL-3 License](./LICENSE).

## Usage

Put the Git Broom executable somewhere in your `PATH`, then invoke it with `git-broom` or `git broom` command.

Without any parameters, it will work in the current directory (which must be a Git repository), checking for merged branches on the current branch.

The branch on which merged branches should be looked for can be specified using the `-b` or `--branch` command line parameter, and the path to a Git repository can be specified as an argument, as shown below:

```
git broom --branch MyBranch ../path/to/my/application/repository
```

## Protected branches

You may have branches that you do not want to delete, even if they are merged. You can define these branches as "protected", as a comma-separated list regular expressions stored with Git configuration under the `broom.protectedbranches` key.

```
git config --local broom.protectedbranches main,release-1.0
```

```
git config --global broom.protectedbranches main,^release/+
```

Branches matching any of the regular expresions will not be deleted by Git Broom. If a protected branch is merged, you will only be informed by the tool.

See https://docs.rs/regex/latest/regex/#syntax for details on the regular expression syntax. 

## Build

Install [Rust](https://www.rust-lang.org/), then run:

```
cargo build
```

For a release build, run:

```
cargo build --release
```

### Building Windows target on Linux

1. Install Cross: `cargo install cross`
2. Install Podman: `apt-get install podman`
3. Install Windows target: `rustup target add x86_64-pc-windows-gnu`
4. Build: `cross build --target x86_64-pc-windows-gnu`

## License

This project is released under the [GPL-3 License](./LICENSE).