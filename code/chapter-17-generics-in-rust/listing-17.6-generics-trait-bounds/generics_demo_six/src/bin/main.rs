use generics_demo_six::{
  print_labeled_value, add_and_log, print_discounted_total
};

fn main() {
  print_labeled_value("CPU Usage", 72);
  print_labeled_value("System Status", "Nominal");
  print_labeled_value("Load Average", 0.85);

  add_and_log(5, 10);         //  15
  add_and_log(3.0, 2.5);      //  5.5

  print_discounted_total("SKU123", 19.99);
  print_discounted_total(456, 50);
  print_discounted_total("GIFT-CARD", 25);
}
