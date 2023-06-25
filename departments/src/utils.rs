use std::collections::HashMap;
use std::io;

pub enum DepartmentWorkers<'a> {
    Workers(Vec<String>, String),
    Err(&'a str),
}

pub fn add_data(mut db: HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    let mut input = String::new();
    println!("Enter the department separated ONLY by a semicolon and then the name.");
    println!("e.g. engineering;Sally");

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input!");

    input = input.trim().to_ascii_uppercase().to_string();
    let input: Vec<&str> = input.split(';').collect();
    let department: String = input[0].to_string();
    let worker: String = input[1].to_string();

    let workers_in_dep = db.get(&department);

    let mut workers_in_dep = match workers_in_dep {
        Some(data) => data.to_vec(),
        None => vec![],
    };

    println!("Worker {} added to {}", worker, department);
    println!("");
    workers_in_dep.push(worker);
    db.insert(department, workers_in_dep);
    db
}

pub fn retrieve_data(db: &HashMap<String, Vec<String>>) -> DepartmentWorkers<'static> {
    let mut department = String::new();

    println!(
        "Which department would you like to inspect? 
    (Hint: type 'all' for all departments)"
    );

    io::stdin()
        .read_line(&mut department)
        .expect("Error reading input!");

    let department = department.trim().to_ascii_uppercase().to_string();

    if department == "ALL" {
        let mut result: Vec<String> = Vec::new();
        for (department_name, workers) in db.iter() {
            let mut workers = workers.to_vec();
            for name in &mut workers {
                *name = format!("{name} --- {department_name}");
            }
            result.append(&mut workers);
        }
        DepartmentWorkers::Workers(result, String::from("ALL DEPARTMENTS"))
    } else {
        match db.get(&department) {
            Some(workers) => DepartmentWorkers::Workers(workers.to_vec(), department),
            None => DepartmentWorkers::Err("Department does not exist 😕"),
        }
    }
}
