pub mod employees {

    use std::collections::HashMap;

    pub fn add_employee(employees: &mut HashMap<String,Vec<String>>, words: &Vec<&str>) {
        for (index, &word) in words.iter().enumerate() {
            if word == "to" {

                if index< words.len()-1 {
                    let employees_in_dep = employees
                        .entry(words[index+1..].join(" "))
                        .or_default();

                    employees_in_dep
                        .push(words[1..index].join(" "));
                    employees_in_dep.sort();

                } else {
                    println!("Department not given!");
                }
            }
        }
    }

    fn list_in_department(employees: &HashMap<String, Vec<String>>,
        department_to_list : String) {
        match employees.get(&department_to_list) {
            None => println!("No employees found for {}",department_to_list),
            Some(list) => {
                println!("-----{}-----",department_to_list);
                for name in list {
                    println!("{}",name);
                }
            }
        }

    }

    fn list_all_by_department(employees: &HashMap<String, Vec<String>>) {
        for (department,names) in employees {
            println!("-------{}--------",department);
            for name in names {
                println!("\t{}",name);
            }
        }
    }

    pub fn list_employees(employees: &mut HashMap<String, Vec<String>>, words: &Vec<&str>) {
        if words.len() < 2 {
            println!("Invalid input.");
            println!("Valid List commands");
            println!("\tList <Department Name>");
            println!("\tList Company");
        } else {
            match words[1] {
                "Company" | "company" => list_all_by_department(employees),
                _ => list_in_department(employees,words[1..].join(" "))
            }
        }
    }

}
