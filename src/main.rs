use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process;

use clap::Parser;

/// Pull the contents of a duplicated nested directory up one level
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to search
    #[arg(default_value = ".", index = 1)]
    path: String,
}

fn move_contents(
    source_dir: std::path::PathBuf,
    current_dir: std::path::PathBuf,
) -> io::Result<()> {
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let path = entry.path();
        fs::rename(path, current_dir.join(entry.file_name()))?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut current_dir = env::current_dir()?;
    let mut move_outside_current = false;
    if args.path != "." {
        current_dir = PathBuf::from(args.path);
        move_outside_current = true;
    }

    let current_dir_name = current_dir.file_name().as_ref().and_then(|s| s.to_str());

    let current_dir_name = match current_dir_name {
        Some(current_dir_name) => current_dir_name,
        None => {
            eprintln!("Issue getting current directory name");
            process::exit(1);
        }
    };

    // Find any directory in the current directory that has the same name as the current directory
    let source_dir = fs::read_dir(current_dir.clone())?
        .filter_map(|entry| {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() && path.file_name().unwrap() == current_dir_name {
                Some(path)
            } else {
                None
            }
        })
        .next();

    match source_dir {
        Some(source_dir) => {
            move_contents(source_dir.clone(), current_dir.clone())?;
            if move_outside_current {
                println!("pulled {:?} to one level to {:?}.", source_dir, current_dir);
            } else {
                println!("pulled {:?} up here.", source_dir);
            }
            fs::remove_dir(source_dir)?;
        }
        None => {
            eprintln!(
                "🧐 there is no directory called '{}' here",
                current_dir_name
            );
            process::exit(1);
        }
    }

    Ok(())
}
