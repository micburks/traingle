use std::io;

use super::query::Query;

pub enum Command {
    Get(Query),
    Add {
        empl_name: String,
        dept_name: String,
    },
    Remove(Query),
    Exit,
}

pub fn get_command() -> Command {
    loop {
        println!("Enter command (get, add, remove, exit):");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        input.trim();
        let mut words = input.split_whitespace();
        let first_word = words.next();
        match first_word {
            Some("get") => {
                break match words.next() {
                    Some("all") => Command::Get(Query::All),
                    None => Command::Get(Query::All),
                    Some(name) => Command::Get(Query::Name(name.to_string())),
                }
            }
            Some("add") => {
                let empl_name = words.next();
                let to = words.next();
                let dept_name = words.next();
                if dept_name == None || to == None || empl_name == None {
                    println!("use `add EMPLOYEE to DEPARTMENT` format.");
                    continue;
                } else if to == Some("to") {
                    break Command::Add {
                        empl_name: empl_name.unwrap().to_string(),
                        dept_name: dept_name.unwrap().to_string(),
                    };
                } else {
                    println!("use `add EMPLOYEE to DEPARTMENT` format.");
                    continue;
                }
            }
            Some("remove") => {
                break match words.next() {
                    Some("all") => Command::Remove(Query::All),
                    None => Command::Remove(Query::All),
                    Some(name) => Command::Remove(Query::Name(name.to_string())),
                }
            }
            Some("exit") => break Command::Exit,
            _ => {
                println!("unrecognized command.");
                continue;
            },
        }
    }
}
