use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let name = "Frank";
    let score = 95;
    let mut file = File::create("data.txt")?;    
    writeln!(file, "Name: {}, Score: {}", name, score)?;
    write!(file, "Hello {}", "Frank")?;
    // a friendlier version of write!()
    file.write_fmt(format_args!("Hello {}", " Frank "))?; 
    Ok(())
}
