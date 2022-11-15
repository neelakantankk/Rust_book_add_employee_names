use add_employee_names::employees;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        println!("What would you like to do?");
        println!("(Type 'Help' to see available commands.)");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let words = input.split_whitespace().collect::<Vec<_>>();

        match words.first() {
            Some(&"add") | Some(&"Add") => employees::add_employee(&mut employees, &words),
            Some(&"List") | Some(&"list") => employees::list_employees(&mut employees, &words),
            Some(&"Quit") | Some(&"quit") | Some(&"q") => break,
            Some(&"Help") | Some(&"help") => {
                println!("Valid commands:");
                println!("\tAdd <Employee Name> to <Department Name>");
                println!("\tList <Department Name>");
                println!("\tList Company");
                println!("\tHelp");
                println!("\tQuit");
            }
            None => continue,
            Some(&&_) => {
                println!("Unknown input");
                println!("Valid commands:");
                println!("\tAdd <Employee Name> to <Department Name>");
                println!("\tList <Department Name>");
                println!("\tList Company");
                println!("\tHelp");
                println!("\tQuit");
            }
        };
    }
}
