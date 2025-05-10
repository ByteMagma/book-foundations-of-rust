// bring in Employee, Manager and Payable from lib.rs
use payable_trait_demo::{Employee, Manager, Payable};
fn main() {
    let emp = Employee {
        name: "Alice".to_string(),
        salary: 52_000.00,
    };
    let mgr = Manager {
        name: "Bob".to_string(),
        salary: 68_000.00,
        bonus: 10_000.00,
    };
    emp.pay();
    mgr.pay();
}
