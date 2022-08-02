use std::collections::HashMap;
use crate::hash_map::Department::Sales;

#[derive(Eq, Hash, PartialEq)]
#[derive(Debug)]
pub enum Department {
    Sales,
    Marketing,
    Management,
}

pub struct Employees {
    employees: HashMap<Department, Vec<String>>,
}

impl Employees {
    pub fn new() -> Employees {
        Employees { employees: HashMap::new() }
    }

    pub fn add_employer(&mut self, name: String, department: Department) {
        let mut employees = self.employees.get_mut(&department);
        match employees {
            None => {
                println!("list wos create and {name} is added to {department:?} department");
                self.employees.insert(department, vec![name]);
            }
            Some(mut vec) => {
                if vec.contains(&name) {
                    println!("person is added already");
                } else {
                    println!("{name} is added to {department:?} department");
                    vec.push(name);
                }
            }
        }
    }

    pub fn get_employers(&self, from: Department) -> Option<&Vec<String>> {
        self.employees.get(&from)
    }

    pub fn get_all_employees(&mut self) -> Vec<&String> {
        let mut filtered_employees: Vec<&String> = Vec::new();
        for value in self.employees.values() {
            for string in value {
                filtered_employees.push(string);
            }
        }
        filtered_employees.sort();
        filtered_employees
    }
}