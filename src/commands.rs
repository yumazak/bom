extern crate regex;
use std::io;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

use termion;
use termion::{color, style};

pub fn add(dir: &Path, target: &Path, ignore: &Vec<String>, root: &str) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let mut root2 = root.to_string();
            let     entry = entry?;
            let     path  = entry.path();
            let     ta    = &target.join(&path.file_name().unwrap());

            root2.push_str(path.file_name().unwrap().to_str().unwrap());

            if path.is_dir() {
                if ignore.contains(&root2) { continue; }
                root2.push_str("/");                
                fs::create_dir(ta)?;
                add(&path, ta, ignore, &root2)?;
            } else {
                if ignore.contains(&root2) { continue; }
                fs::copy(&path, ta)?;
            }
        }
    }
    Ok(())
}

pub fn get_ignore(dir: &Path, root: &Path) -> io::Result<Vec<String>> {
    let mut v: Vec<String>  = vec![];
    let mut v2: Vec<String> = vec![];
    let mut s               = String::new();
    let     path            = dir.join(".bomignore");

    let mut file = match File::open(&path) {
        Err(_) => { return Ok(v) },
        Ok(file) => file,
    };

    let mut env_ignore = match File::open(&root) {
        Err(_) => {
            println!("error");
            return Ok(v)
        },
        Ok(file) => file,
    };

    match file.read_to_string(&mut s) {
        Err(_) => {},
        Ok(_)  => v = s.split_whitespace().map(|s| s.to_string()).collect()
    }
    match env_ignore.read_to_string(&mut s) {
        Err(_) => {},
        Ok(_)  => {
            v2 = s.split_whitespace().map(|s| s.to_string()).collect();
            v.extend(v2.iter().cloned());
        }
    }
    Ok(v)
} 
pub fn delete(path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path  = entry.path();

        if path.is_dir() {
            delete(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }

    match fs::remove_dir_all(path){
        Ok(_)    => {},
        Err(err) => println!("{}", err),
    }

    Ok(())
}
// -> io::Result<Vec<String>>
pub fn get_projects (path: &Path) -> io::Result<Vec<String>> {
    let mut projects = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path  = entry.path();
        
        if path.is_dir() {
            match path.file_name().unwrap().to_str() {
                Some(project) => projects.push(project.to_string()),
                None          => println!("error")
            }
        } else {
            let project = path.file_name().unwrap().to_str().unwrap().to_string();
            projects.push(project);
        }
    }

    Ok(projects)
}

pub fn show_projects_with_key_position<W: Write> (screen: &mut W, positon: usize, projects: Vec<String>) {
    let position_x = 4;
    let offset_y   = 4;

    write!(screen, "{}{}{}", termion::clear::All, termion::cursor::Goto(0, 1), termion::cursor::Hide);
    println!("\nBoilerplate List\n");

    for i in 0..projects.len() {
        write!(screen, "{}{}", termion::cursor::Goto(position_x, (i + offset_y) as u16), termion::cursor::Hide);
        
        if i == positon {
            println!("‣{} {}{}", color::Fg(color::Green), projects[i], style::Reset);
        } else {
            println!(" {}", projects[i]);
        }
    }

    screen.flush().unwrap();
}

pub fn list(path: &Path) -> io::Result<()> {
    let mut has_boiler = false;

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path  = entry.path();

        if path.is_dir() {
            has_boiler = true;

            match path.file_name().unwrap().to_str() {
                Some(name) => println!("  ‣ {}", name),
                _          => println!("can't read boiler"),
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
        let path  = entry.path();

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

pub fn ignore_list(ignore_path: &Path) -> io::Result<()> {
    let mut s = String::new();
    let mut v: Vec<String> = vec![];
    
    let mut global_ignore = match File::open(&ignore_path) {
        Err(why) => { panic!("{}", why); },
        Ok(file) => file,
    };

    match global_ignore.read_to_string(&mut s) {
        Err(why) => { panic!("{}", why); },
        Ok(_)    => {
            v = s.split_whitespace().map(|s| s.to_string()).collect();
            for name in &v {
                println!("  ‣ {}", name)
            }
        }
    }

    Ok(())
}

pub fn ignore_add(ignore_path: &Path, name: String) -> io::Result<()> {
    let mut v: Vec<String> = vec![];
    let mut s              = String::new();
    
    match File::open(&ignore_path) {
        Err(why)     => panic!("{}", why),
        Ok(mut file) => {
            match file.read_to_string(&mut s) {
                Err(why) => panic!("{}", why),
                Ok(_)    => v = s.split_whitespace().map(|s| s.to_string()).collect()
            }
        }
    };

    fs::remove_file(ignore_path)?;

    let mut buffer = File::create(ignore_path)?;

    v.push(name);

    for n in &v {
        buffer.write_all(n.as_bytes())?;
        buffer.write_all(b"\n")?;
    }

    Ok(())
}

pub fn ignore_remove(ignore_path: &Path, name: String) -> io::Result<()> {
    let mut v: Vec<String> = vec![];
    let mut s              = String::new();
    
    match File::open(&ignore_path) {
        Err(why)     => panic!("{}", why),
        Ok(mut file) => {
            match file.read_to_string(&mut s) {
                Err(why) => panic!("{}", why),
                Ok(_)    => v = s.split_whitespace().map(|s| s.to_string()).collect()
            }
        }
    };
    if !v.contains(&name) { return Ok(()); }

    fs::remove_file(ignore_path)?;

    let mut buffer = File::create(ignore_path)?;

    for n in &v {
        if(n.to_string() == name) { continue; }
        
        buffer.write_all(n.as_bytes())?;
        buffer.write_all(b"\n")?;
    }

    Ok(())
}