fn main() {
    let var_one = true;
    let var_two = false;
    let var_three = true;
    
    println!("should be true: {}", var_one && var_three);
    println!("should be false: {}", var_one && var_two);    
    println!("should be true: {}", var_one || var_two);
    println!("should be true: {}", var_two || var_three);
    println!("should be true: {}", var_one && !var_two);
    println!("should be false: {}", !var_one || !var_three);   
}
