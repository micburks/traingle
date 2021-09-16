use super::command::Command;
use super::query::Query;

pub struct Company {
    // arguably, departments have unique names. HashMap may be more appropriate
    departments: Vec<Department>
}

impl Company {
    pub fn new() -> Company {
        Company {
            departments: Vec::new(),
        }
    }
    pub fn run(&mut self, cmd: Command) {
        match cmd {
            Command::Get(query) => {
                if let Query::Name(name) = query {
                    self.print_dept(name);
                } else {
                    self.print_all();
                }
            }
            Command::Add{ empl_name, dept_name } => self.add(empl_name, dept_name),
            Command::Remove(query) => {
                if let Query::Name(name) = query {
                    self.remove_matching(name);
                } else {
                    self.remove_all();
                }
            },
            Command::Exit => (), // will never be called
        }
    }
    fn print_all(&self) {
        println!(" --- get all! ---");
        for dept in &self.departments {
            dept.print();
        }
    }
    fn print_dept(&self, dept_name: String) {
        println!(" --- get {}! ---", dept_name);
        for dept in &self.departments {
            if dept.name == dept_name {
                dept.print();
                break;
            }
        }
    }
    fn add(&mut self, empl_name: String, dept_name: String) {
        println!(" --- add! ---");
        let mut has_dept = false;
        for dept in &mut self.departments {
            if dept.name == dept_name {
                has_dept = true;
                dept.employees.push(Employee::new(&empl_name));
                break;
            }
        }
        if !has_dept {
            let mut new_dept = Department::new(&dept_name);
            new_dept.employees.push(Employee::new(&empl_name));
            self.departments.push(new_dept);

        }
        println!("added");
    }
    fn remove_matching(&mut self, name: String) {
        println!(" --- remove {}! ---", name);
        for dept in &mut self.departments {
            dept.employees.retain(|empl| empl.name != name);
        }
    }
    fn remove_all(&mut self) {
        println!(" --- remove all! ---");
        for dept in &mut self.departments {
            dept.employees = Vec::new();
        }
    }
}

struct Department {
    name: String,
    employees: Vec<Employee>,
}

impl Department {
    fn new(name: &String) -> Department {
        Department {
            name: name.to_string(),
            employees: Vec::new(),
        }
    }
    fn print(&self) {
        println!("{}", self.name);
        for empl in &self.employees {
            println!(" - {}", empl.name);
        }
    }
}

struct Employee {
    name: String,
}

impl Employee {
    fn new(name: &str) -> Employee {
        Employee {
            name: name.to_string()
        }
    }
}
