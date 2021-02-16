use std::string::String;
use std::collections::HashMap;


pub struct employee_manage {
    data: Vec<String>,
    employee_department_map: HashMap<String, String>
}


impl employee_manage {

    pub fn new() -> employee_manage {
        employee_manage {
            data: Vec::new(),
            employee_department_map: HashMap::new()
        }
    }


    pub fn add_employee(&mut self, name: String, department: String) {
        self.data.push(name.clone());
        self.employee_department_map.insert(name, department);
    }

    pub fn list_all(&self) {
        for (key, value) in &self.employee_department_map {
            println!("Key: {}, Value: {}", key, value);
        }
    }




}