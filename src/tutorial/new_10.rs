// Error handling in Rust & File I/O in Rust using Result

use std::fs::File;
use std::io::{Write, BufReader, BufRead, ErrorKind};

pub fn my_error() {
    // Result has 2 variants: Ok and Err
    // enum Result<T,E> {
    //     Ok(T),
    //     Err(E)
    // }
    // where T represents the data type of the value returned and E the type of error

    // Specify the file path
    let path: &str = "lines.txt";

    // Attempt to create a new file
    let output = File::create(path);
    // Match the result of creating a file
    let mut output = match output {
        Ok(file) => file, // File successfully created
        Err(error) => panic!("Problem creating file: {:?}", error), // Error occurred while creating the file
    };

    // Write content to the file
    write!(output, "Just some Random words \nHello akib").expect("Failed to write file!!");

    // Open the file for reading
    let input = File::open(path).unwrap();
    // Create a buffered reader to read lines from the file
    let buffered = BufReader::new(input);

    // Iterate over each line in the file and print it
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    // Attempt to create another file
    let output2 = File::create("rand.txt");
    // Match the result of creating the second file
    let output2 = match output2 {
        Ok(file) => file, // File successfully created
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc, // File created after handling NotFound error
                Err(e) => panic!("Can't create file: {:?}", e), // Error occurred while creating the file
            },
            _other_error => panic!("Problem opening file: {:?}", error), // Other types of errors
        },
    };
}
