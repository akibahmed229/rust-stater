use std::collections::HashMap;

pub fn tuple(){
    /*
     * Tuples are like structured containers that hold multiple values of different types. 
     * They are created using parentheses and can hold any number of elements.
     * Tuples have a fixed size and each element can be accessed by index.
     * Useful for grouping related data and returning multiple values from functions.
     */

    let my_tuples:(u8, String, f64) = (10, "Akib".to_string(), 3.14);
    println!("Tuple: {:?}", my_tuples);

    let (x, y, z) = my_tuples; // destructuring the tuple 
    println!("Tuple: {} {} {}", x, y, z);
}

pub fn my_vectors () {
    /*
     * Vectors in Rust are dynamic, growable arrays that allow you to store multiple values of the same type in contiguous memory.
     * They provide flexibility by automatically handling memory allocation and deallocation, allowing them to resize dynamically 
     * as elements are added or removed. Vectors are versatile and convenient, offering a wide range of methods for manipulation and access.
     * They are commonly used when the size of the collection is not known in advance or may change dynamically during program execution.
     */

    // & creates references, enabling borrowing without ownership transfer.
    // * dereferences references, allowing access to the value they point to.

    // Create an empty vector using Vec::new() and initialize another vector with initial values
    let vec1: Vec<i32> = Vec::new();
    let mut vec2 = vec![1, 9, 2, 3, 4, 10, 6];

    // Add an element to the vector
    vec2.push(5);

    // Print the contents of the vector
    println!("Vector 2: {:?}", vec2);

    // Access the second element of the vector using indexing
    let second: &i32 = &vec2[1];
    match vec2.get(1) {
        // Some and None are variants of the Option enum, which is commonly used to represent the possibility of absence or presence of a value. 

        // If the element exists at the specified index, print it
        Some(second) => println!("2nd: {}", second),
        // If the element does not exist at the specified index, print a message indicating its absence
        None => println!("No 2nd value"),
    }

    // Multiply each element of the vector by 2 using mutable iteration
    for i in &mut vec2 {
        *i *= 2;
    }

    // Print the modified vector
    for i in &vec2 {
        print!("{} ", i);
    }
    println!();

    // Print the length of the vector
    println!("Length of vector 2 is: {}", vec2.len());

    // Remove and return the last element of the vector
    println!("Pop: {:?}", vec2.pop());
}

pub fn my_hashmap() {
    // A HashMap in Rust is like a dictionary or a map in other programming languages. It stores key-value pairs, where each key must be unique. 

    // Create a new empty HashMap with keys and values of type &str
    let mut names: HashMap<&str, &str> = HashMap::new();

    // Insert key-value pairs into the HashMap
    names.insert("Akib", "love rust");
    names.insert("Atoshi", "love python");
    names.insert("Yeanur", "love java");

    // Iterate over key-value pairs in the HashMap and print them
    for (key, value) in names.iter() {
        println!("{} = {}", key, value);
    }

    // Print the length of the HashMap
    println!("Length of names: {}", names.len());

    // Check if a key exists in the HashMap
    if names.contains_key(&"Akib") {
        // Get the value associated with the key "Akib"
        let the_name = names.get(&"Akib");

        // Match the result of getting the value
        match the_name {
            Some(value) => println!("Akib {}", value),
            None => println!("Not Found!!!"),
        }
    }
}

