use std::env;
use std::ffi::OsStr;
use std::fs::{self};
use std::io::Result;
use std::path::PathBuf;

fn rename_files_in_folder(dir: &PathBuf) -> Result<()> {
    let files = fs::read_dir(dir.clone())?;

    let mut i = 0;
    for dir_entry in files {
        let dir_entry_uw = dir_entry.unwrap();
        println!("-- found file: {}", dir_entry_uw.path().display());
        let path = dir_entry_uw.path();

        if path.is_dir() {
            rename_files_in_folder(&path)?; // recurse
            continue;
        }

        // Make sure the extension doesn't change
        let extension = match path.extension() {
            Some(str) => str,
            None => OsStr::new(""),
        };

        //println!("- extension: {}", extension.to_str().unwrap());

        let mut new_path = dir.clone();
        new_path.push(format!("{}.{}", i, extension.to_str().unwrap()));

        println!(
            "-- Renaming: {} to {}.{}",
            new_path.as_path().display(),
            i,
            extension.to_str().unwrap()
        );

        fs::rename(dir_entry_uw.path(), new_path.as_path())?;

        i += 1;
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let cur = env::current_dir()?;

    let mut target_dir = cur.clone();

    if cfg!(debug_assertions) {
        target_dir.push("test");
    }

    println!("READING DIR: {}", target_dir.display());

    rename_files_in_folder(&target_dir)?;

    Ok(())
}
