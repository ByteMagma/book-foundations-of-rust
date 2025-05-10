pub trait Payable {
    fn get_salary(&self) -> f32; // Required method
    fn pay(&self) {              // Default method
        println!("You got paid: {}", self.get_salary() / 52.0);
    }
}

pub struct Employee {
    pub name: String,
    pub salary: f32,
}
pub struct Manager {
    pub name: String,
    pub salary: f32,
    pub bonus: f32,
}

impl Payable for Employee {
    fn get_salary(&self) -> f32 {
        self.salary
    }
    // uses default pay()
}
impl Payable for Manager {
    fn get_salary(&self) -> f32 {
        self.salary
    }
    // custom implementation of the pay() method
    fn pay(&self) {
        let total = (self.get_salary() / 52.0) + (self.bonus / 52.0);
        println!(
            "Manager {} got paid (with bonus): {}", 
            self.name, 
            total
        );
    }
}
