// Using a hash map and vectors, create a text interface to allow a
// user to add employee names to a department in a company; for example,
// “Add Sally to Engineering” or “Add Amir to Sales.”
//
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io::{self};

enum Command {
    Add {
        employee: String,
        department: String,
    },
    List {
        department: Option<String>,
    },
    Quit,
}

fn parse_command(input: &str) -> Result<Command, &'static str> {
    let mut parts = input.trim().split_whitespace();

    match parts.next() {
        Some("add") => {
            let employee = parts.nth(0).ok_or("Missing employee name")?.to_string();
            let to = parts.next().ok_or("Missing 'to' keyword.")?;
            if to != "to" {
                return Err("Expected 'to' keyword.");
            }
            let department = parts.next().ok_or("Missing department name.")?.to_string();
            Ok(Command::Add {
                employee,
                department,
            })
        }
        Some("list") => {
            let department = parts.next().map(|s| s.to_string());
            Ok(Command::List { department })
        }
        Some("quit") => Ok(Command::Quit),
        _ => Err("Invalid command."),
    }
}

fn handle_add(ledger: &mut HashMap<String, Vec<String>>, employee: &str, department: &str) {
    ledger
        .entry(department.to_string())
        .and_modify(|e| e.push(employee.to_string()))
        .or_insert(vec![employee.to_string()]);
}

fn handle_list(ledger: &HashMap<String, Vec<String>>, department: Option<String>) {
    match department {
        Some(d) => {
            let employees = ledger.get(&d).unwrap();
            let mut sorted_employees = employees.clone();
            sorted_employees.sort();
            println!("{:?}", sorted_employees);
        }
        None => {
            println!("{:?}", ledger);
        }
    }
}

fn main() {
    let mut ledger: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("Input your command (add/list/quit):");
        let mut instruction = String::new();

        io::stdin()
            .read_line(&mut instruction)
            .expect("Can't read line.");

        let command = parse_command(&instruction);

        match command {
            Ok(Command::Add {
                employee,
                department,
            }) => {
                handle_add(&mut ledger, &employee, &department);
            }
            Ok(Command::List { department }) => {
                handle_list(&ledger, department);
            }
            Ok(Command::Quit) => {
                println!("Exiting program!");
                break;
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
}
