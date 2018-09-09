extern crate regex;
use std::io;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

use termion;
use termion::{color, style};

pub fn add(relative_target_project_path: &Path, full_target_path: &Path, ignore_file_names: &Vec<String>, global_ignore_path: &str) -> io::Result<()> {
    if relative_target_project_path.is_dir() {
        for entry in fs::read_dir(relative_target_project_path)? {
            let mut root2 = global_ignore_path.to_string();
            let     entry = entry?;
            let     path  = entry.path();
            let     ta    = &full_target_path.join(&path.file_name().unwrap());

            root2.push_str(path.file_name().unwrap().to_str().unwrap());

            if path.is_dir() {
                if ignore_file_names.contains(&root2) { continue; }
                root2.push_str("/");                
                fs::create_dir(ta)?;
                add(&path, ta, ignore_file_names, &root2)?;
            } else {
                if ignore_file_names.contains(&root2) { continue; }
                fs::copy(&path, ta)?;
            }
        }
    }
    Ok(())
}

//addでもinitでも呼ばれる。 => boilerplateでもprojectでも使われる
//.bomignoreがなければ空の配列を返す
pub fn get_ignore(target_project_path: &Path, global_ignore_path: &Path) -> io::Result<Vec<String>> {
    let mut ignore_file_names: Vec<String>        = vec![];
    let mut global_ignore_file_names: Vec<String> = vec![];
    let mut file_names                            = String::new();
    let     target_project_ignore_path            = target_project_path.join(".bomignore");

    // let mut project_ignore_file: File;
    // let mut global_ignore_file:  File;

    match File::open(&target_project_ignore_path) {
        Err(_)   => {},
        Ok(mut project_ignore_file) => {
            if project_ignore_file.read_to_string(&mut file_names).is_ok() {
                ignore_file_names = file_names.split_whitespace().map(|file_names| file_names.to_string()).collect()
            } else {
                println!("Can't read {:?}", target_project_path);
            }
        }
    };

    match File::open(&global_ignore_path) {
        Err(_)   => {},
        Ok(mut global_ignore_file) => {
            if global_ignore_file.read_to_string(&mut file_names).is_ok() {
                global_ignore_file_names = file_names.split_whitespace().map(|file_names| file_names.to_string()).collect();
                ignore_file_names.extend(global_ignore_file_names.iter().cloned());
            } else {
                println!("Can't read {:?}", global_ignore_path)
            }
        }
    };

    Ok(ignore_file_names)
} 

pub fn delete(target_project_path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(target_project_path)? {
        let entry = entry?;
        let target_project_path = entry.path();

        if target_project_path.is_dir() {
            delete(&target_project_path)?;
        } else {
            fs::remove_file(&target_project_path)?;
        }
    }

    fs::remove_dir_all(target_project_path)?;

    Ok(())
}
// -> io::Result<Vec<String>>
pub fn get_projects (target_project_path: &Path) -> io::Result<Vec<String>> {
    let mut projects = Vec::new();

    for entry in fs::read_dir(target_project_path)? {
        let entry = entry?;
        let target_project_path  = entry.path();
        
        if target_project_path.is_dir() {
            match target_project_path.file_name().unwrap().to_str() {
                Some(project) => projects.push(project.to_string()),
                None          => println!("Error")
            }
        } else {
            let project = target_project_path.file_name().unwrap().to_str().unwrap().to_string();
            projects.push(project);
        }
    }

    Ok(projects)
}

pub fn show_projects_with_key_position<W: Write> (screen: &mut W, cursor_positon: usize, projects: Vec<String>) {
    let position_x = 4;
    let offset_y   = 4;
    write!(screen, "{}", termion::cursor::Restore);  
    write!(screen, "{}", termion::clear::AfterCursor);
    screen.flush().unwrap();

    // write!(screen, "{}", termion::cursor::Down(1));
    write!(screen, "{}", termion::cursor::Save);
    // write!(screen, "{}", termion::cursor::Hide);

    println!("Boilerplate List");
    write!(screen, "{}{}", termion::cursor::Restore, termion::cursor::Down(1));
    for i in 0..projects.len() {
        write!(screen, "{}", termion::cursor::Down(1));
        write!(screen, "{}", termion::cursor::Save);
        write!(screen, "{}", termion::cursor::Right(4));

        if i == cursor_positon {
            println!("‣{} {}{}", color::Fg(color::Green), projects[i], style::Reset);
        } else {
            println!(" {}", projects[i]);
        }
        write!(screen, "{}", termion::cursor::Restore);  
    }

    write!(screen, "{}", termion::cursor::Up(projects.len() as u16 + 1));
    write!(screen, "{}", termion::cursor::Save);

    screen.flush().unwrap();
}

pub fn list(target_project_path: &Path) -> io::Result<()> {
    let mut has_boiler = false;

    for entry in fs::read_dir(target_project_path)? {
        let entry = entry?;
        let target_project_path  = entry.path();

        if target_project_path.is_dir() {
            has_boiler = true;

            match target_project_path.file_name().unwrap().to_str() {
                Some(name) => println!("  ‣ {}", name),
                _          => println!("can't read boiler"),
            }
        } else {
            println!("{:?}", target_project_path.file_name().unwrap());
        }
    }
    
    if !has_boiler {println!("  Nothing boilerplates")};

    Ok(())
}

pub fn has_boiler(boiler_name: &str, boilerplates_path: &Path) -> io::Result<(bool)> {
    for entry in fs::read_dir(boilerplates_path)? {
        let entry = entry?;
        let boilerplates_path  = entry.path();

        if boilerplates_path.is_dir() {
            match boilerplates_path.file_name().unwrap().to_str() {
                Some(name) => {
                    if name == boiler_name {
                        return Ok(true)
                    }
                },
                _ => println!("Can't read boiler"),
            }
        } else {
            println!("{:?}", boilerplates_path.file_name().unwrap());
        }
    }

    Ok(false)
}

pub fn ignore_list(global_ignore_path: &Path) -> io::Result<()> {
    let mut s = String::new();
    let mut v: Vec<String> = vec![];
    
    let mut global_ignore = File::open(&global_ignore_path)?;

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

pub fn ignore_add(global_ignore_path: &Path, target_ignore_file_name: String) -> io::Result<()> {
    let mut v: Vec<String> = vec![];
    let mut s              = String::new();
    
    match File::open(&global_ignore_path) {
        Err(why)     => panic!("{}", why),
        Ok(mut file) => {
            match file.read_to_string(&mut s) {
                Err(why) => panic!("{}", why),
                Ok(_)    => v = s.split_whitespace().map(|s| s.to_string()).collect()
            }
        }
    };

    fs::remove_file(global_ignore_path)?;

    let mut buffer = File::create(global_ignore_path)?;

    v.push(target_ignore_file_name);

    for n in &v {
        buffer.write_all(n.as_bytes())?;
        buffer.write_all(b"\n")?;
    }

    Ok(())
}

pub fn ignore_remove(global_ignore_path: &Path, target_ignore_file_name: String) -> io::Result<()> {
    let mut v: Vec<String> = vec![];
    let mut s              = String::new();
    
    match File::open(&global_ignore_path) {
        Err(why)     => panic!("{}", why),
        Ok(mut file) => {
            match file.read_to_string(&mut s) {
                Err(why) => panic!("{}", why),
                Ok(_)    => v = s.split_whitespace().map(|s| s.to_string()).collect()
            }
        }
    };
    if !v.contains(&target_ignore_file_name) { return Ok(()); }

    fs::remove_file(global_ignore_path)?;

    let mut buffer = File::create(global_ignore_path)?;

    for n in &v {
        if(n.to_string() == target_ignore_file_name) { continue; }
        
        buffer.write_all(n.as_bytes())?;
        buffer.write_all(b"\n")?;
    }

    Ok(())
}