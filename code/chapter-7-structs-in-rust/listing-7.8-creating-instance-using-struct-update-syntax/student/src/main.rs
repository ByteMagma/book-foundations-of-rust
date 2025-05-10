#[derive(Debug)]
struct Student {
    active: bool,
    name: String,
    email: String,
    age: u8,
}

fn main() {
    let mut student_one = Student {
        active: true,
        name: String::from("Susan Carson"),
        email: String::from("susan_carson@xyzcompany.com"),
        age: 24,
    };

    println!("{:?}", student_one);
    student_one.age = 23;
    println!("{:?}", student_one);

    let student_two = build_student(
        true, 
        String::from("Tim Weston"), 
        String::from("tweston@greatmail.com"), 
        32
    );

    let student_three = Student {
        name: String::from("Ronald Jones"),
        email: String::from("ronald@small_company.com"),
        age: 45,
        ..student_two
    };    
}

fn build_student(
    active: bool, 
    name: String, 
    email: String, 
    age: u8
) -> Student {
    Student {
        active,
        name,
        email,
        age,
    }
}
