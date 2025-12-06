use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {

    // make a 100 number array?
    let mut combo: Vec<u8> = Vec::new();
    for i in 0..100 {
        // println!("{i}");
        combo.push(i);
    }

    // dbg!(combo);
    // dbg!(combo.len());

    let pointer = 0; // i will use this to keep track of where we pointed to in the combo vector
    let password = 0; // the final password will be incremented if we pass over a zero

    let file_path = Path::new("input.txt"); // Specify the path to your file

    // Open the file and create a buffered reader
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Iterate over the lines in the file
    for line_result in reader.lines() {
        let line = line_result?; // Handle potential errors during line reading
        // println!("{}", line); // Process or print each line
        if line.len() > 0 {
        if line.as_bytes()[0] == b'L'  {
            println!("FOUND A L!!")
        }}

    }

    Ok(())
}

