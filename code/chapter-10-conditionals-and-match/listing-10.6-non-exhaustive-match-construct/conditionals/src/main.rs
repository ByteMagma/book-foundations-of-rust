enum Direction {
    North,
    South,
    East,
    West,
}
fn describe_direction(dir: Direction) {
    match dir {
        Direction::North => println!("Heading North"),
        Direction::South => println!("Going South"),
        Direction::East => println!("Moving toward East"),
    }
}
fn main() {
    describe_direction(Direction::West);
    describe_direction(Direction::South);
    describe_direction(Direction::North);
    describe_direction(Direction::East);
}
