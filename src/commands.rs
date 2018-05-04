extern crate regex;
use std::io;
use std::fs::{self, File};
use std::mem;
use std::io::{BufReader, Read};
use std::path::Path;
use self::regex::Regex;
use std::env;

pub fn add(dir: &Path, target: &Path, ignore: &Vec<String>, root: &str) -> io::Result<()> {
    // let re = Regex::new(r"[^/]*").unwrap();
    // let text = "test/target/hello";
    // for cap in re.captures_iter(text) {
    //     println!("file path is {:?}", &cap[0]);
    // }
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let ta = &target.join(&path.file_name().unwrap());
            let mut root2 = root.to_string();
            let name = path.file_name().unwrap();            
            root2.push_str("/");
            root2.push_str(path.file_name().unwrap().to_str().unwrap());
                println!("target is {:?}", root2);
                println!("ignores is {:?}", ignore);
            
            if path.is_dir() {
                if ignore.contains(&root2) {continue;}
                fs::create_dir(ta);
                add(&path, ta, ignore, &root2)?;
            } else {
                if ignore.contains(&root2)  {continue;}
                fs::copy(&path, ta)?;
            }
        }
    }
    Ok(())
}

// fn reg(str: String) -> String {
//     let re = Regex::new(r"[/*(^[/]+)/*]+").unwrap();
// }
pub fn get_ignore(dir: &Path, root: &Path) -> Vec<String>{
    let path = dir.join(".bomignore");
    println!("path is {:?}", path);
    // let path = Path::new(".gitignore");
    let display = path.display();
    let mut s = String::new();    
    let mut v: Vec<String> = vec![];
    let mut v2: Vec<String> = vec![];

    let mut file = match File::open(&path) {
        Err(why) => {return v},
        Ok(file) => file,
    };
    println!("{:?}", &root);
    let mut env_ignore = match File::open(&root) {
        Err(why) => {println!("error");return v},
        Ok(file) => file,
    };

    match file.read_to_string(&mut s) {
        Err(why) => {},
        Ok(_) => {
            // v.push(s.split_whitespace().collect());
            v = s.split_whitespace().map(|s| s.to_string()).collect();
        }
    }
    match env_ignore.read_to_string(&mut s) {
        Err(why) => {},
        Ok(_) => {
            // v.push(s.split_whitespace().collect());
            v2 = s.split_whitespace().map(|s| s.to_string()).collect();
            v.extend(v2.iter().cloned());
        }
    }
    v
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
                Some(name) => println!("  ‣ {}", name),
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