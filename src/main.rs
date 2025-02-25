use std::{path::PathBuf, process};

use anyhow::Context;
use clap::Parser;
use regex::{NoExpand, Regex};

/// Rename files matching a regular expression by replacing parts of their name.  Similar to the
/// util-linux `rename` command, but with support of regular expressions.
#[derive(Debug, Clone, Parser)]
#[clap(rename_all_env = "kebab-case")]
struct Cli {
    /// Prevent Regex parts from being expanded (i.e., `$1`, `$name`)
    #[clap(short, long)]
    no_expand: bool,
    /// Perform no filesystem operations and display to the user the changes that would happen
    /// without the flag
    #[clap(long, conflicts_with = "quiet")]
    dry_run: bool,
    /// Don't echo the renmaes to STDOUT
    #[clap(short, long)]
    quiet: bool,

    // NYI
    // /// Rename all files within the directory provided
    // #[clap(short, long)]
    // dir: bool,
    // /// Recurse into subdirectories
    // #[clap(short, long, requires = "dir")]
    // recursive: bool,
    /// Only replace the first match in the file name
    #[clap(short, long)]
    first: bool,

    /// Regex to use to search in the string.  Automatically does a global search.  Note: only the
    /// file names are matched against.  Any preceding directory will be ignored.
    #[clap(name = "regex")]
    regex: Regex,
    /// String to replace matches with.  This will exapand `$1` and `$name` into the groups matched
    /// by the regex provided.  If this is not the desired behaviour, `--no-expand` should be used.
    #[clap(name = "replacement")]
    replacement: String,
    /// Files to attempt to rename
    #[clap(required = true, name = "files")]
    files: Vec<PathBuf>,

    // Add "--copy" flag to copy files to destination instead of moving them.
    #[arg(short, long)]
    copy: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut renames = Vec::with_capacity(cli.files.len());
    for file in cli.files {
        let name = file.file_name().context("Path must have file name")?;
        let name = name.to_str().context("File name is not valid Unicode")?;

        let new = if cli.no_expand {
            let rep = NoExpand(&cli.replacement);
            if cli.first {
                cli.regex.replace(name, rep)
            } else {
                cli.regex.replace_all(name, rep)
            }
        } else {
            let rep = &cli.replacement;
            if cli.first {
                cli.regex.replace(name, rep)
            } else {
                cli.regex.replace_all(name, rep)
            }
        };

        let new = new.into_owned();

        let new_path = file
            .parent()
            .context("file path does not have parent")?
            .join(new);

        // Collect into vec so that if one failes, then no changes are made
        renames.push((file, new_path));
    }

    let mut renamed = false;
    for (from, to) in &renames {
        if from == to {
            if !cli.quiet {
                println!("No change for '{}'", from.display());
            }
            continue;
        }

        if !cli.quiet {
            if cli.copy {
                println!("'{}' copied -> '{}'", from.display(), to.display());
            } else {
                println!("'{}' -> '{}'", from.display(), to.display());
            }
        }

        if !cli.dry_run {
            if cli.copy {
                std::fs::copy(from, to).with_context(|| {
                    format!("Cannot copy file {} -> {}", from.display(), to.display())
                })?;
            } else {
                std::fs::rename(from, to).with_context(|| {
                    format!("Cannot rename file {} -> {}", from.display(), to.display())
                })?;
            }
        }
        renamed = true;
    }

    if !renamed {
        // If we didn't rename anything, give an error status
        process::exit(1);
    }

    Ok(())
}
