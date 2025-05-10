use std::fmt::Display;

pub struct Wrapper<T> {
    pub value: T,
}

// General methods that don't require trait bounds
impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }

    pub fn replace(&mut self, new_value: T) {
        self.value = new_value;
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn from_other<U>(other: U) -> Wrapper<U> {
        Wrapper { value: other }
    }    
}

// Methods that require T to implement Display
impl<T: Display> Wrapper<T> {
    pub fn describe(&self) {
        println!("The current value is: {}", self.value);
    }
}
