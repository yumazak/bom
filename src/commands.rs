use std::io;
use std::fs::{self};
use std::path::Path;

pub fn add(dir: &Path, target: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let ta = &target.join(&path.file_name().unwrap());
            if path.is_dir() {
                if path.file_name().unwrap() == ".git" || path.file_name().unwrap() == "node_modules" { continue; }                   
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
        Ok(_) => {},
        Err(err) => println!("{}", err),
    }
    Ok(())
}

pub fn list(path: &Path) -> io::Result<()> {
    let mut has_boiler = false;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            has_boiler = true;
            match path.file_name().unwrap().to_str() {
                Some(name) => println!("  ‣{}", name),
                _ => println!("can't read boiler"),
            }
        } else {
            println!("{:?}", path.file_name().unwrap());
        }
    }
    if !has_boiler {println!("  Nothing boilerplates")};
    Ok(())
}

pub fn has_boiler(boiler_name: &str, path: &Path) -> io::Result<(bool)> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            match path.file_name().unwrap().to_str() {
                Some(name) => {
                    if name == boiler_name {
                        return Ok(true)
                    }
                },
                _ => println!("can't read boiler"),
            }
        } else {
            println!("{:?}", path.file_name().unwrap());
        }
    }
    Ok(false)
}