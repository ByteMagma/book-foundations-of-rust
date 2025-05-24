use std::collections::HashMap;

fn main() {
    let students = HashMap::from([
        ("Alice", [85, 90, 93, 88]),
        ("Bob", [78, 82, 89, 91]),
        ("Charlie", [92, 95, 87, 90]),
        ("David", [80, 85, 88, 84]),
        ("Eve", [95, 92, 90, 93]),
    ]);

    for (name, grades) in &students {
        let total: u32 = grades.iter().sum();
        let average = total as f32 / grades.len() as f32;
        println!("{}'s average grade is {:.2}", name, average);
    }
}
