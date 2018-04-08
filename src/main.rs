#[macro_use]
extern crate clap;
mod cli;
mod commands;
use std::fs;
use std::path::Path;

use std::env;
fn main() {
    let matches = cli::build_cli().get_matches();
    let mut path;
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
                Err(err) => println!("Error: {}", err),
            }
            println!("{:?}", &target);
            match commands::add(Path::new(o), &target){
                Ok(_) => println!("success"),
                Err(err) => println!("err"),
            };
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("rm") {
        if let Some(name) = matches.value_of("name") {
            target = path.join(name);
            match commands::delete(&target) {
                Ok(_) => println!("success"),
                Err(err) => println!("err"),
            }
        }
    }
}