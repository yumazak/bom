#[macro_use]
extern crate clap;
extern crate termion;

mod cli;
mod commands;
use std::fs;
use std::path::Path;
use std::env;
use std::io::{self, Write, stdin, stdout};

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let matches = cli::build_cli().get_matches();
    let path;
    let root;
    let mut target;
    match env::home_dir() {
        Some(p) => {
            path = p.join(".bom/boilerplates");
            root = p.join(".bom/.bomignore");
        }
        None => panic!("Impossible to get your home dir!"),
    }
    fs::create_dir(&path).expect("can't create dir");

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

            if matches.is_present("force") {
                match fs::read_dir(&target) {
                    Ok(_) => {
                        match commands::delete(&target) {
                            Ok(_) => {},
                            Err(err) => println!("{}", err),
                        }
                    }
                    Err(err) => {println!("{}", err)}
                }
            }

            fs::create_dir(&target).expect("can't create dir");

            match commands::add(Path::new(o), &target, &commands::get_ignore(Path::new(o), &root).expect("Can't get ignore"), ""){
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
        let mut boiler_name    = String::new();
        let mut project_name   = String::new();
        let mut pressed_ctrl_c = false;

        if matches.is_present("interactive") {
            let mut stdout           = stdout().into_raw_mode().unwrap();
            let mut cuurent_position = 0;
            let     stdin            = stdin();
            let     projects         = commands::get_projects(&path).unwrap();

            commands::show_projects_with_key_position(&mut stdout, cuurent_position, projects.clone());
            for c in stdin.keys() {
                match c.unwrap() {
                    Key::Char('\n') => break,
                    Key::Ctrl('c') => {
                        // write!(stdout, "{}", termion::cursor::Goto(0, 1));
                        // stdout.flush().unwrap();
                        pressed_ctrl_c = true;
                        // std::process::exit(0);
                        break;
                    }
                    Key::Down => {
                        if cuurent_position < projects.len() - 1 { cuurent_position += 1; }
                        commands::show_projects_with_key_position(&mut stdout, cuurent_position, projects.clone());
                    }

                    Key::Up => {
                        if cuurent_position > 0 { cuurent_position -= 1 }
                        commands::show_projects_with_key_position(&mut stdout, cuurent_position, projects.clone());
                    }

                    _ => {}
                }
                stdout.flush().unwrap();
            }

            let stdin                = io::stdin();
            let mut stdin            = stdin.lock();
            let mut is_started_input = false;
            boiler_name              = projects[cuurent_position].clone();
            if pressed_ctrl_c { return; }

            print!("{}Project name: {}{}", termion::cursor::Goto(0, 5 + projects.len() as u16), &boiler_name, termion::cursor::Show);

            //input Project name
            stdout.flush().unwrap();
            for c in stdin.keys() {
                match c.unwrap() {
                    Key::Char('\n') => break,
                    Key::Ctrl('c') => {
                        pressed_ctrl_c = true;
                        break;
                    }
                    Key::Char(c) => {
                        if !is_started_input {
                            is_started_input = true;
                            let line = format!("Project name: {}", boiler_name);
                            print!("{}{}", termion::clear::CurrentLine, termion::cursor::Left(line.len() as u16));
                            print!("Project name: ");
                        }
                        print!("{}", c);
                        project_name.push(c);
                    }
                    Key::Backspace => {
                        if project_name.is_empty() {
                            continue;
                        }
                        project_name.pop();
                        let line = format!("Project name: {}", project_name);
                        print!("{}{}{}", termion::clear::CurrentLine, termion::cursor::Left(line.len() as u16 + 1), line);
                    }
                    _ => {}
                }
                stdout.flush().unwrap();
            }

            if pressed_ctrl_c { return; }

            stdout.flush().unwrap();

            if !is_started_input { project_name = boiler_name.clone() }
        }

        if boiler_name.is_empty() {
            if let Some(name) = matches.value_of("boiler_name") {
                boiler_name = name.to_string();
            } else { //名前を受け取らなかったら

            }
        }

        if project_name.is_empty() {
            if let Some(name) = matches.value_of("project_name") {
                project_name = name.to_string();
            }
        }

        if commands::has_boiler(&boiler_name, &path).unwrap() {
            if project_name == "." {
                target = env::current_dir().unwrap();
            } else {
                target = Path::new(&project_name).to_path_buf();
            }
            match fs::create_dir(&target) {
                Ok(_) => {},
                Err(_) => {},
            }
            match commands::add(&path.join(&boiler_name), &target, &commands::get_ignore(&path.join(&boiler_name), &root).expect("Can't get ignore"), ""){
                Ok(_) => println!("success"),
                Err(_) => println!("Error"),
            };
        } else {
            println!("can't find {}", boiler_name);
        }
    }

    //ignore
    if let Some(ref matches) = matches.subcommand_matches("ignore") {
        if let Some(_) = matches.subcommand_matches("ls") {
            println!("\n Global ignore files\n");
            match commands::ignore_list(&root) {
                Ok(_) => {},
                Err(err) => println!("{}", err),
            }
            println!("");
        }
        if let Some(matches) = matches.subcommand_matches("add") {
            if let Some(n) = matches.value_of("name") {
                match commands::ignore_add(&root, n.to_string()) {
                    Ok(_) => {println!("add {} to ignore list", n)},
                    Err(err) => println!("{}", err),
                }
            } else {
                println!("nothing");
            }
  
            println!("");
        }
        if let Some(matches) = matches.subcommand_matches("rm") {
            if let Some(n) = matches.value_of("name") {
                match commands::ignore_remove(&root, n.to_string()) {
                    Ok(_) => {println!("remove {} from ignore list", n)},
                    Err(_) => println!("can't find {}", n),
                }
            } else {
                println!("nothing");
            }
  
            println!("");
        }
    }

}