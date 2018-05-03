extern crate regex;
use std::io;
use std::fs::{self, File};
use std::mem;
use std::io::{BufReader, Read};
use std::path::Path;
use self::regex::Regex;
use std::env;

pub fn add(dir: &Path, target: &Path, ignore: &Vec<String>) -> io::Result<()> {
    // let re = Regex::new(r"[^/]*").unwrap();
    // let text = "test/target/hello";
    // for cap in re.captures_iter(text) {
    //     println!("file path is {:?}", &cap[0]);
    // }
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            // let re1 = Regex::new(env::current_dir().unwrap().as_path()).unwrap();
            // for cap in re1.captures_iter(path.to_str().unwrap()) {
            //     println!("file path is {:?}", &cap);
            // }
            // let re = Regex::new(r"[([^[\\]]*)]+").unwrap();
            println!("{:?}", ignore);
            println!("{}", env::current_dir().unwrap().as_path().to_str().unwrap().replace("\\", "/"));
            println!("{}", path.to_str().unwrap().replace("\\", "/"));
            // if ignore.contains(path.to_str().unwrap().replace("\\", "/")) {
            //     println!("{:?} is ignore", path);
            // }
            let ta = &target.join(&path.file_name().unwrap());
            if path.is_dir() {
                if path.file_name().unwrap() == ".git" || path.file_name().unwrap() == "node_modules" { continue; }                   
                fs::create_dir(ta);
                add(&path, ta, ignore)?;
            } else {
                fs::copy(&path, ta)?;
            }
        }
    }
    Ok(())
}

// fn reg(str: String) -> String {
//     let re = Regex::new(r"[/*(^[/]+)/*]+").unwrap();
// }
pub fn get_ignore() -> Vec<String>{
    let path = Path::new(".gitignore");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}", display),
        Ok(file) => file,
    };

    let mut s = String::new();
    let mut v: Vec<String> = vec![];    
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}", display),
        Ok(_) => {
            // v.push(s.split_whitespace().collect());
            v = s.split_whitespace().map(|s| s.to_string()).collect();
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