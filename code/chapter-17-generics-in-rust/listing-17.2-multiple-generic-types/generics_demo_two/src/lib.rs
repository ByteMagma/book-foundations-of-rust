pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}

pub fn create_pair<T, U>(a: T, b: U) -> Pair<T, U> {
    Pair { first: a, second: b }
}
