fn main() {
    let user_count = 100;
    just_print(user_count);
    change_and_print(user_count);
}

fn just_print(count: i32) {
    println!("User count: {}", count);
}

fn change_and_print(mut count: i32) {
    count *= 2;
    println!("User count: {}", count);
}

