use std::env;
use std::fs;
use std::io;
use std::process;

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
    // Get the current directory name
    let current_dir = env::current_dir()?;
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
            println!("moving {:?} up here...", source_dir);
            move_contents(source_dir.clone(), current_dir.clone())?;
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
