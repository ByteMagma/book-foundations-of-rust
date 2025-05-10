fn main() {
    let point = (3, 5, 8);
    match point {
        (0, my_y, and_z) => 
            println!("Point on the x-axis at y = {}, z = {}", my_y, and_z),
        (x_elem, 0, z_elem) => 
            println!("Point on the y-axis at x = {}, z = {}", x_elem, z_elem),
        (x, y, 0) => 
            println!("Point on the z-axis at x = {}, y = {}", x, y),
        (x_data, y_elem, z_last) => 
            println!("Point at ({}, {}, {})", x_data, y_elem, z_last),
    }
}
