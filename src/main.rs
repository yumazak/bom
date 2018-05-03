#[macro_use]
extern crate clap;
mod cli;
mod commands;
use std::fs;
use std::path::Path;
use std::env;
fn main() {
    let matches = cli::build_cli().get_matches();
    let path;
    let mut target;
    match env::home_dir() {
        Some(p) => path = p.join(".bom/boilerplates"),
        None => panic!("Impossible to get your home dir!"),
    }
    fs::create_dir(&path);
    //add
    if let Some(ref matches) = matches.subcommand_matches("add") {
        if let Some(o) = matches.value_of("url") {
            if let Some(n) = matches.value_of("name") {
                target = path.join(n);
            } else {
                if o == "." {
                    target = path.join(env::current_dir().unwrap().as_path().file_name().unwrap());
                } else {
                    target = path.join(Path::new(o).file_name().unwrap());
                }
            }
            match fs::create_dir(&target) {
                Ok(_) => {},
                Err(err) => println!("{}", err),
            }
            match commands::add(Path::new(o), &target, &commands::get_ignore()){
                Ok(_) => println!("success"),
                Err(err) => println!("{}", err),
            };
        }
    }

    //remove
    if let Some(ref matches) = matches.subcommand_matches("rm") {
        if let Some(name) = matches.value_of("name") {
            target = path.join(name);
            match commands::delete(&target) {
                Ok(_) => println!("removed {}", name),
                Err(err) => println!("{}", err),
            }
        }
    }
    
    //list
    if let Some(_) = matches.subcommand_matches("ls") {
        println!("\nBoilerplate List\n");
        match commands::list(&path) {
            Ok(_) => {},
            Err(err) => println!("{}", err),
        }
        println!("");
    }

    //init
    if let Some(ref matches) = matches.subcommand_matches("init") {
        if let Some(boiler_name) = matches.value_of("boiler_name") {
            if commands::has_boiler(boiler_name, &path).unwrap() {
                if let Some(project_name) = matches.value_of("project_name") {
                    if project_name == "." {
                        target = env::current_dir().unwrap();
                    } else {
                        target = Path::new(project_name).to_path_buf();
                    }
                } else {
                    target = Path::new(boiler_name).to_path_buf();
                }
                match fs::create_dir(&target) {
                    Ok(_) => {},
                    Err(_) => {},
                }
                match commands::add(&path.join(boiler_name), &target, &commands::get_ignore()){
                    Ok(_) => println!("success"),
                    Err(_) => println!("Error"),
                };
            } else {
                println!("can't find {}", boiler_name);
            }
        }
    }
}