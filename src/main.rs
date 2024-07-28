use clap::error::Result;
use clap::{command, Parser, Subcommand};
use is_executable::IsExecutable;
use std::io::{self};
use std::{env, path::PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// show which .fun dir is being used
    #[arg(short, long, default_value_t = false)]
    dir: bool,

    #[command(subcommand)]
    script: Option<Comands>,
    // #[arg(trailing_var_arg(true))]
    // script_args: Option<Vec<String>>,
}

#[derive(Debug, Subcommand)]
enum Comands {
    #[command(external_subcommand)]
    Script(Vec<String>),
}

#[derive(thiserror::Error, Debug)]
enum Error {
    #[error("Could not find fun directory")]
    NoFunDir,

    #[error("Script {0} does not exist")]
    ScriptNotFound(PathBuf),

    #[error("{0:?} is not a file")]
    ScriptNotFile(PathBuf),

    #[error("invalid symbolic link chain starting at {0} ending at {1}")]
    InvalidSymbolicLink(PathBuf, PathBuf),

    #[error("Script is not executable ({0})")]
    ScriptNotExecutable(PathBuf),

    #[error("IO fuckup")]
    IoError(#[from] io::Error),
}

fn main() {
    match run() {
        Err(e) => eprint!("{}\n", e.to_string()),
        Ok(_) => {}
    }
}

fn run() -> Result<(), Error> {
    let args = Args::parse();

    // println!("{args:?}");

    let dir = get_fun_dir()?;

    if args.dir {
        println!("{}", dir.display());
        return Ok({});
    }

    let mut path = dir.to_owned();
    let sargs = match args.script {
        None => return Ok(()),
        Some(script) => match script {
            Comands::Script(v) => v,
        },
    };

    path.push(&sargs[0]);
    let sargs = &sargs[1..];

    if !path.exists() {
        return Err(Error::ScriptNotFound(path));
    }

    let mut md = std::fs::metadata(&path)?;
    if md.is_symlink() {
        let npath = std::fs::canonicalize(&path)?;
        if !npath.exists() {
            return Err(Error::InvalidSymbolicLink(path, npath));
        }
        path = npath;
        md = std::fs::metadata(&path)?;
    }

    if !md.is_file() {
        return Err(Error::ScriptNotFile(path));
    }

    if !path.is_executable() {
        return Err(Error::ScriptNotExecutable(path));
    }

    let mut handle = std::process::Command::new(&path).args(sargs).spawn()?;

    handle.wait()?;

    Ok({})
}

fn get_fun_dir() -> Result<PathBuf, Error> {
    let mut dir = env::current_dir()?;

    dir.push(".fun");

    while !dir.is_dir() {
        dir.pop();
        dir.pop();
        let parent = dir.parent();
        if let None = parent {
            return Err(Error::NoFunDir);
        }
        dir.push(".fun");
    }

    Ok(dir)
}
