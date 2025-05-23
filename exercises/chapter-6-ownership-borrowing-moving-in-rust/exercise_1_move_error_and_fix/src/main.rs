fn main() {
    let department = String::from("Engineering");
    let work_area = department.clone();

    println!("Department: {}", department);

    let department = String::from("Engineering");
    let work_area = &department;

    println!("Department: {}", department);
}
