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

#[derive(Debug)]
enum ListResult {
    SingleDeparment(Vec<String>),
    WholeCompany(HashMap<String, Vec<String>>),
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
        .or_default()
        .push(employee.to_string())
}

fn handle_list(ledger: &HashMap<String, Vec<String>>, department: Option<String>) -> ListResult {
    match department {
        Some(dept) => {
            let employees = ledger.get(&dept).unwrap();
            let mut sorted_employees = employees.clone();
            sorted_employees.sort();
            println!("{:?}", sorted_employees);
            ListResult::SingleDeparment(sorted_employees)
        }
        None => {
            let mut sorted_company: HashMap<String, Vec<String>> = HashMap::new();
            let mut sorted_departments: Vec<String> = ledger.keys().cloned().collect();
            sorted_departments.sort();

            for dept in &sorted_departments {
                let mut sorted_employees = ledger[dept].clone();
                sorted_employees.sort();
                println!("{}: {:?}", dept, sorted_employees);
                sorted_company.insert(dept.to_string(), sorted_employees);
            }
            ListResult::WholeCompany(sorted_company)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_employees_to_empty_department() {
        let mut ledger: HashMap<String, Vec<String>> = HashMap::new();
        let employee = "Jack";
        let department = "Engineering";

        handle_add(&mut ledger, employee, department);

        assert_eq!(ledger[department], vec![employee]);
    }

    #[test]
    fn adds_employees_to_existing_deparments() {
        let mut ledger: HashMap<String, Vec<String>> = HashMap::new();
        let employee = "Amy";
        let department = "Sales";
        ledger.insert(department.to_string(), vec!["Bob".to_string()]);

        handle_add(&mut ledger, employee, department);

        assert_eq!(ledger[department], vec!["Bob", employee])
    }

    #[test]
    fn lists_employees_in_a_deparment_alphabetically() {
        let department = String::from("Engineering");
        let ledger: HashMap<String, Vec<String>> = HashMap::from([(
            department.clone(),
            vec![
                String::from("Paulo"),
                String::from("Lucas"),
                String::from("Andre"),
            ],
        )]);

        match handle_list(&ledger, Some(department)) {
            ListResult::SingleDeparment(employees) => {
                assert_eq!(employees, vec!["Andre", "Lucas", "Paulo"]);
            }
            ListResult::WholeCompany(_) => {
                panic!(
                    "Unexpected return from handle_list, expected SingleDepartment, got WholeCompany"
                )
            }
        }
    }

    #[test]
    fn lists_all_departments_alphabetically() {
        let department = String::from("Engineering");
        let department2 = String::from("Sales");
        let ledger: HashMap<String, Vec<String>> = HashMap::from([
            (
                department.clone(),
                vec![
                    String::from("Paulo"),
                    String::from("Lucas"),
                    String::from("Andre"),
                ],
            ),
            (
                department2.clone(),
                vec![String::from("Amy"), String::from("Alice")],
            ),
        ]);

        match handle_list(&ledger, None) {
            ListResult::SingleDeparment(_) => {
                panic!(
                    "Unexpected return from handle_list, expected WholeCompany, got SingleDepartment"
                )
            }
            ListResult::WholeCompany(company) => {
                assert_eq!(
                    company,
                    HashMap::from([
                        (
                            String::from("Engineering"),
                            vec![
                                String::from("Andre"),
                                String::from("Lucas"),
                                String::from("Paulo"),
                            ]
                        ),
                        (
                            String::from("Sales"),
                            vec![String::from("Alice"), String::from("Amy"),]
                        )
                    ])
                );
            }
        }
    }
}
