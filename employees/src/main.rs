// Using a hash map and vectors, create a text interface to allow a
// user to add employee names to a department in a company; for example,
// “Add Sally to Engineering” or “Add Amir to Sales.”
//
// Then let the user retrieve a list of all people in a department
// or all people in the company by department, sorted alphabetically.

use std::collections::HashMap;
use std::io::{self};

fn main() {
    let mut ledger: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("Input your command:");
        let mut instruction = String::new();

        io::stdin()
            .read_line(&mut instruction)
            .expect("Can't read line.");

        if instruction.starts_with("list") {
            let mut parts = instruction.trim().split_whitespace();
            let _ = parts.next();
            let department = parts.next();
            if department.is_some() {
                let employees = ledger.get(department.unwrap()).unwrap();
                let mut sorted_employees = employees.clone();
                sorted_employees.sort();
                println!("{:?}", sorted_employees);
            } else {
                println!("{:?}", ledger);
            }
        } else {
            let mut parts = instruction.split_whitespace().into_iter();
            let employee = parts.nth(1).unwrap();
            let department = parts.nth(1).unwrap();

            ledger
                .entry(department.to_string())
                .and_modify(|e| e.push(employee.to_string()))
                .or_insert(vec![employee.to_string()]);
        }
    }
}
