use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

pub fn add(dir: &Path, target: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let ta = &target.join(&path.file_name().unwrap());
            if path.is_dir() {
                if(path.file_name().unwrap() == ".git") {
                    continue;
                }
                fs::create_dir(ta);
                add(&path, ta)?;
            } else {
                fs::copy(&path, ta)?;
            }
        }
    }
    Ok(())
}

pub fn delete(path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            delete(&path);
        } else {
            fs::remove_file(&path)?;
        }
    }
    match fs::remove_dir_all(path){
        Ok(_) => println!("success"),
        Err(err) => println!("{}", err),
    }
    Ok(())
}