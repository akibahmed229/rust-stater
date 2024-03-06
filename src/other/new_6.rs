// Stack:
// - Stores data in a last-in, first-out (LIFO) manner.
// - Used for storing local variables and function call information.
// - Variables have fixed size known at compile time.
// - Automatic memory allocation and deallocation by the Rust compiler.
// - Access is fast and efficient due to predictable memory management.
// - Limited in size compared to heap memory.

// Heap:
// - Stores data with dynamic size and lifetime.
// - Used for dynamically allocated objects like vectors, strings, etc.
// - Variables have size determined at runtime and can grow or shrink.
// - Explicit memory allocation and deallocation by the programmer.
// - Access involves dereferencing pointers and can be slightly slower than stack.
// - Larger in size compared to stack and dynamically managed.


// RULES:
// 1. Each value has a variable that's called it's owner 
// 2. There is only one owner at a time 
// 3. when the value go out of the scope the value disappears


// Define a function `print_str` that takes ownership of a `String` and prints it
fn print_str(str: String) {
    println!("A string {}", str);
}

// Define a function `print_return_str` that takes ownership of a `String`, prints it, and returns it
fn print_return_str(str: String) -> String {
    println!("A string {}", str);
    return str;
}

// Define a function `change_str` that takes a mutable reference to a `String`, modifies it, and returns a new `String`
fn change_str(str: &mut String) -> String {
    str.push_str(" is happy");
    return str.to_string();
}

// Define a function `my_ownership` to demonstrate ownership and borrowing in Rust
fn my_ownership() {
    // Create a new `String` str1
    let str1: String = String::from("Akib");

    // Clone str1 to create str2
    let str2 = str1.clone(); // Cloned str1, so str1 remains valid
    println!("Hello {}", str1); // str1 is still valid after cloning

    // Create a mutable `String` str3 and example of ownership transfer
    let mut str3 = String::from("Magi");
    // Pass a mutable reference to str3 to the change_str function, which modifies it
    let str4 = change_str(&mut str3);
    println!("{}", str4); // Print the modified str3
    println!("{}", str3); // str3 has been modified

    // Create a new `String` str5
    let str5 = String::from("Space");
    // Call print_return_str, which takes ownership of str5, prints it, and returns it
    let str6 = print_return_str(str5);
    println!("{}", str6); // Print the returned str5
}
