#[macro_use]
extern crate clap;
mod cli;
mod add;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};
fn main() {
    let matches = cli::build_cli().get_matches();
    if let Some(ref matches) = matches.subcommand_matches("add") {
        let path = Path::new("./boilerplates");
        let mut target;
        match fs::create_dir(&path) {
            Ok(_) => {},
            Err(err) => {},
        }
        if let Some(o) = matches.value_of("url") {
            if let Some(n) = matches.value_of("name") {
                target = path.join(n);
                println!("add {:?}", &target);
            } else {
                target = path.join(Path::new(o).file_name().unwrap());
            }
            println!("{:?}", &target);
            match fs::create_dir(&target) {
                Ok(_) => {println!("path is {:?}", &target)},
                Err(err) => println!("Error: {}", err),
            }
            add::visit_dirs(Path::new(o), &target); 
        }
    }
    

}