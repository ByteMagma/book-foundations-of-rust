/* A Rust program that creates an array of characters and prints it.
    The program defines four characters, stores them in an array, and prints the array.
    The output will be: ['G', 'r', 'e', 'g']
*/
fn main() {
    let c1: char = 'G';
    let c2: char = 'r';
    let c3: char = 'e';
    let c4: char = 'g';
    let array_of_characters: [char; 4] = [c1, c2, c3, c4];
    println!("{:?}", array_of_characters);
}
