use std::env;
use std::ffi::OsStr;
use std::fs;

//fn wait() {}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let cur = env::current_dir()?;

    let mut target_dir = cur.clone();

    if cfg!(debug_assertions) {
        target_dir.push("test");
    }

    println!("READING DIR: {}", target_dir.display());

    let files = fs::read_dir(target_dir.clone())?;

    let mut i = 0;
    for dir_entry in files {
        let dir_entry_uw = dir_entry.unwrap();
        println!("-- found file: {}", dir_entry_uw.path().display());
        let path = dir_entry_uw.path();

        if path.is_dir() {
            continue;
        }

        // Make sure the extension doesn't change
        let extension = match path.extension() {
            Some(str) => str,
            None => OsStr::new(""),
        };

        //println!("- extension: {}", extension.to_str().unwrap());

        let mut new_path = target_dir.clone();
        new_path.push(format!("{}.{}", i, extension.to_str().unwrap()));

        println!(
            "-- attempting to rename to: {}",
            new_path.as_path().display()
        );

        fs::rename(dir_entry_uw.path(), new_path.as_path())?;

        i += 1;
    }

    Ok(())
}
