fn inform_admin(employee_name: &str, employee_age: u8) {
    println!("Hey admin, buy a cake for {}, they are {} years old!", employee_name, employee_age);
}

fn main() {
    let name = "Alice";
    let age = 30;
    inform_admin(name, age);
}
