#[derive(Debug)]
struct Student {
    first_name: String,
    last_name: String,
    age: u32,
    major: String,
}

fn main() {
    let student = Student {
        first_name: String::from("John"),
        last_name: String::from("Doe"),
        age: 20,
        major: String::from("Computer Science"),
    };

    let full_name = student.first_name.clone() + " " + &student.last_name;

    println!("Full Name: {}", full_name);
    println!("{:?}", student);
}
