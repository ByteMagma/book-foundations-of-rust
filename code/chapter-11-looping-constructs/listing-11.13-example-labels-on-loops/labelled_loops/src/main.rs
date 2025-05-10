fn main() {
    let warehouse = vec![
        ("A", vec!["item1", "fragile_item", "item2"]),
        ("B", vec!["item3", "urgent_order", "item4"]),
        ("C", vec!["item5", "item6", "fragile_item"]),
    ];
    let current_temp = 32; // Current warehouse temperature
    'section_loop: for (section, items) in warehouse.iter() {
        println!("Inspecting section: {}", section);
        for item in items {
            if current_temp > 30 && item.contains("fragile") {
                println!("  Skipping fragile item due to high temperature.");
                continue;
            }
            println!("  Checking item: {}", item);

            if item == &"urgent_order" {
                println!("  High-priority item found! Halting search.");
                break 'section_loop;
            }
        }
    }
    println!("Inspection complete.");
}
