fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    // Using iter() - Borrowing the elements immutably
    for num in numbers.iter() {
        println!("Immutable reference: {}", num);
    }
    // Using iter_mut() - Modifying elements in place
    for num in numbers.iter_mut() {
        *num *= 2;
    }
    println!("After iter_mut: {:?}", numbers);
    // Using into_iter() - Taking ownership and consuming
    for num in numbers.into_iter() {
        println!("Owned value: {}", num);
    }
    // Error: numbers is no longer accessible after into_iter()
    // println!("{:?}", numbers);
}
