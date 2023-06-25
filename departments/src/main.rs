mod utils;

use std::collections::HashMap;
use std::io;

use crate::utils::{add_data, retrieve_data, DepartmentWorkers};

fn main() {
    let mut db: HashMap<String, Vec<String>> = HashMap::new();
    println!("Welcome to MadeUpCompany Inc!");

    loop {
        println!("Enter 1 to add, 2 to retrieve data, or 3 to exit:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input!");

        if input.trim() == "1" {
            db = add_data(db);
        } else if input.trim() == "2" {
            let department_workers: DepartmentWorkers = retrieve_data(&db);

            let (mut workers, department) = match department_workers {
                DepartmentWorkers::Workers(workers, department) => (workers, department),
                DepartmentWorkers::Err(err) => {
                    println!("{err}");
                    println!("");
                    continue;
                }
            };

            workers.sort();
            println!("Here are the workers in {department}:");
            for worker in &workers {
                println!("{}", *worker);
            }
            println!("");
        } else {
            println!("");
            break;
        }
    }
}
