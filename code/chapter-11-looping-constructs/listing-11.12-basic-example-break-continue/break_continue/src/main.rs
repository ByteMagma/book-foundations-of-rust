fn main() {
    let students = vec!["Alice", "Bob", "Absent", "Charlie", "Principal", "Eve"];
    println!("Checking attendance...");

    for student in students.iter() {
        if student == &"Absent" {
            println!("Skipping: {} (not present)", student);
            continue;
        }
        if student == &"Principal" {
            println!("Principal has arrived. Ending attendance check.");
            break;
        }

        println!("Recording attendance for: {}", student);
    }
    println!("Attendance check complete.");
}
