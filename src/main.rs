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
use termion::{color, style};

fn main() {
    let matches = cli::build_cli().get_matches();
    let boilerplates_path;
    let global_ignore_path;
    let mut target_project_path;

    match env::home_dir() {
        Some(home_path) => {
            let bom_path = home_path.join(".bom");
            if(!bom_path.exists()) {fs::create_dir(bom_path);}

            boilerplates_path  = home_path.join(".bom/boilerplates");
            global_ignore_path = home_path.join(".bom/.bomignore");
        }
        None => panic!("Impossible to get your home dir!"),
    }

    if(!&boilerplates_path.exists()) {
        fs::create_dir(&boilerplates_path);
    }

    if(!&global_ignore_path.exists()) {
        fs::File::create(&global_ignore_path); 
    }

    //add
    if let Some(ref matches) = matches.subcommand_matches("add") {
        if let Some(arg_target_project_path) = matches.value_of("Target Project path") {
            if let Some(n) = matches.value_of("New Boilerplate name") {
                target_project_path = boilerplates_path.join(n);
            } else {
                if arg_target_project_path == "." {
                    target_project_path = boilerplates_path.join(env::current_dir().unwrap().as_path().file_name().unwrap());
                } else {
                    target_project_path = boilerplates_path.join(Path::new(arg_target_project_path).file_name().unwrap());
                }
            }

            if matches.is_present("force") {
                match fs::read_dir(&target_project_path) {
                    Err(err) => {},
                    Ok(_)    => commands::delete(&target_project_path).unwrap()
                }
            }
            fs::create_dir(&target_project_path).expect("Can't create dir");

            match commands::add(Path::new(arg_target_project_path), &target_project_path, &commands::get_ignore(Path::new(arg_target_project_path), &global_ignore_path).expect("Can't get ignore"), "") {
                Ok(_)    => println!("\nFinish"),
                Err(why) => panic!("{}", why),
            }

        }
    }

    //remove
    if let Some(ref matches) = matches.subcommand_matches("rm") {
        if let Some(target_boilerplate_name) = matches.value_of("Boilerplate name") {
            target_project_path = boilerplates_path.join(target_boilerplate_name);

            match commands::delete(&target_project_path) {
                Ok(_)    => println!("Removed {}", target_boilerplate_name),
                Err(err) => panic!("{}", err),
            }
        }
    }
    
    //list
    if let Some(_) = matches.subcommand_matches("ls") {
        println!("\nBoilerplate List\n");
        
        commands::list(&boilerplates_path).unwrap();
        print!("\n");
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
            let     projects         = commands::get_projects(&boilerplates_path).unwrap();

            commands::show_projects_with_key_position(&mut stdout, cuurent_position, projects.clone());
            for c in stdin.keys() {
                match c.unwrap() {
                    Key::Char('\n') => break,

                    Key::Ctrl('c')  => {
                        pressed_ctrl_c = true;
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
            }

            let stdin                = io::stdin();
            let mut stdin            = stdin.lock();
            let mut is_started_input = false;
            boiler_name              = projects[cuurent_position].clone();

            print!("{}", termion::cursor::Show);
            if pressed_ctrl_c { return; }
            
            //Boilerplatet Listと改行があるので + 3している (マジックナンバーどうにかしたい)
            print!("{}Project name: {}{}{}", termion::cursor::Down(projects.len() as u16 + 3), color::Fg(color::LightBlack), &boiler_name, style::Reset);
            print!("{}", termion::cursor::Left(boiler_name.len() as u16));

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
                        if project_name.is_empty() { continue; }

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
            if let Some(boiler_name_arg) = matches.value_of("Boilerplate name") {
                println!("{}", boiler_name_arg);
                boiler_name = boiler_name_arg.to_string();
            }
        }

        if project_name.is_empty() {
            if let Some(project_name_arg) = matches.value_of("Project name") {
                println!("{}", project_name_arg);
                project_name = project_name_arg.to_string();
            } else {
                project_name = boiler_name.clone();
            }
        }

        if commands::has_boiler(&boiler_name, &boilerplates_path).unwrap() {
            if project_name == "." {
                target_project_path = env::current_dir().unwrap();
            } else {
                target_project_path = Path::new(&project_name).to_path_buf();
            }

            fs::create_dir(&target_project_path).expect("Can't create dir");
            match commands::add(&boilerplates_path.join(&boiler_name), &target_project_path, &commands::get_ignore(&boilerplates_path.join(&boiler_name), &global_ignore_path).unwrap(), ""){
                Ok(_)  => println!("\nSuccess"),
                Err(_) => println!("\nError"),
            };
        } else {
            println!("Can't find {}", boiler_name);
        }
    }

    //ignore
    if let Some(ref matches) = matches.subcommand_matches("ignore") {
        if let Some(_) = matches.subcommand_matches("ls") {
            println!("\n Global ignore files\n");
            commands::ignore_list(&global_ignore_path).unwrap();
            print!("\n");
        }

        if let Some(matches) = matches.subcommand_matches("add") {
            if let Some(ignore_file_name) = matches.value_of("Add target ignore file name") {
                match commands::ignore_add(&global_ignore_path, ignore_file_name.to_string()) {
                    Ok(_)    => println!("Add {} to ignore list", ignore_file_name),
                    Err(err) => panic!("{}", err),
                }
            } else {
                println!("Nothing");
            }
  
            print!("\n");
        }

        if let Some(matches) = matches.subcommand_matches("rm") {
            if let Some(ignore_file_name) = matches.value_of("Remove target ignore file name") {
                match commands::ignore_remove(&global_ignore_path, ignore_file_name.to_string()) {
                    Ok(_)  => println!("Remove {} from ignore list", ignore_file_name),
                    Err(_) => println!("Can't find {}", ignore_file_name),
                }
            } else {
                println!("Nothing");
            }
  
            print!("\n");
        }
    }

}