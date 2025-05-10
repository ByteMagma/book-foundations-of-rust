use std::fmt::Display;

pub struct Wrapper<T> {
    pub value: T,
}

impl<T: Display> Wrapper<T> {
    pub fn print(&self) {
        println!("Wrapped value: {}", self.value);
    }

    pub fn new(value: T) -> Self {
        Wrapper { value }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
    }
}
