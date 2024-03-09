// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime 
// &'a mut i32 // a mutable reference with an explicit lifetime
fn example1() {
   let foo = 100;
   let mut r;
    {
         let x = 5; // x is a local variable with a shorter lifetime
         r = &x;    // r references x, borrowing its value
         println!("r: {}", *r);
    }
    r =  &foo;     // r now references foo, which has a longer lifetime than x
    println!("r: {}", *r);
}

// Function to find the longest string
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
   }
}

fn example2() {
    let s1 = String::from("abcd");
    let mut result;
    {
    let s2 = String::from("xyz");
    result = longest(s1.as_str(), s2.as_str()); // Both s1 and s2 are borrowed in the call to longest
    }
    result = longest(s1.as_str(), "wakff"); // s1 is borrowed again
    println!("The longest string is {}", result);
}

// Function to create a new String
fn longest2<'a>(s1: &'a str, s2: &'a str) -> String {
    String::from("New String") // Returns a new String with a local scope
}

fn example3() {
    let s1 = String::from("abcd");
    let mut result;
    {
    let s2 = String::from("jsa");
    result = longest2(s1.as_str(), s2.as_str()); // Both s1 and s2 are borrowed in the call to longest2
    }
    result = longest2(s1.as_str(), "asd"); // s1 is borrowed again
    println!("The longest string is {}", result);
}

// Struct with a reference to a string slice
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn example4() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence }; // 'i' holds a reference to the first sentence of 'novel'

    println!("i: {}", i.part);
}

// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters.
// Function to find the first word in a string
fn first_world<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { //  In Rust, b' ' is a byte literal representing the ASCII value of a space character.
            return &s[0..i]; // Returns a slice of 's' up to the first space
        }
    }
    &s[..] // Returns the entire string if no space is found
}

const S: &'static str = "I have a static lifetime."; // 'static is a special lifetime that lasts for the entire duration of the program.

pub fn my_lifetime() {
    example2(); // Demonstrates borrowing multiple strings with the same lifetime
    example3(); // Demonstrates borrowing multiple strings with different lifetimes
    example4(); // Demonstrates struct with a reference to a string slice
    let val = first_world("Hello World");
    println!("First word: {}", val); // Demonstrates function with lifetime parameter
}
