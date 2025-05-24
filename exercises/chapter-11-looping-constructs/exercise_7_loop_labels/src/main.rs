fn main() {
    let seats = vec![
        vec!["Occupied", "Occupied", "Occupied"],
        vec!["Occupied", "Empty",    "Occupied"],
        vec!["Occupied", "Occupied", "Occupied"],
    ];

    'search: for (row_idx, row) in seats.iter().enumerate() {
        for (col_idx, &seat) in row.iter().enumerate() {
            if seat == "Empty" {
                println!("Found an empty seat at row {}, column {}", row_idx, col_idx);
                break 'search; // Break out of BOTH loops
            }
        }
    }

    println!("Done checking seats.");
}
