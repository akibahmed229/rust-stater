pub fn my_iterator() {
    // Define a 2D array of integers
    let mut arr2d: [[i32; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

    // Iterate over each row in the 2D array
    for row in arr2d.iter() {
        // Iterate over each element in the row
        for col in row.iter() {
            // Print each element followed by a space
            print!("{} ", col);
        }
        // Print a new line after printing all elements in the row
        println!();
    }

    // Create an iterator over the 2D array
    let mut iter = arr2d.iter();

    // Print the iterator
    println!("{:?}", iter);
}

// Declare a public function named your_closure
pub fn your_closure() {
    // Closures in Rust are anonymous functions that can capture variables
    // from their surrounding environment

    // Define a closure named can_vote that takes an i32 age as input
    // and returns a boolean indicating if the person can vote
    let can_vote = |age: i32| -> bool {
        age >= 18
    };

    // Print the result of calling can_vote with age 17 (false)
    println!("{}", can_vote(17)); // Output: false

    // Declare a mutable variable x with initial value 10
    let mut x = 10;

    // Define a closure named print_x that has no arguments
    // and prints the current value of x
    let print_x = || {
        println!("value of x: {}", x);
    };

    // Call the print_x closure to print the value of x
    print_x(); // Output: value of x: 10

    // Define a generic function named use_func that takes three arguments:
    // a: i32, b: i32, and a closure of type T that takes two i32 arguments
    // and returns an i32
    fn use_func<T>(a: i32, b: i32, func: T) -> i32
    where
        T: Fn(i32, i32) -> i32, // Constraint: func must be a closure with specified signature
    {
        // Call the provided closure func with a and b as arguments
        // and return its result
        func(a, b)
    }

    // Define closures for addition and subtraction
    let add = |a: i32, b: i32| -> i32 {
        a + b
    };
    let sub = |a: i32, b: i32| -> i32 {
        a - b
    };

    // Call use_func with different closures for addition and subtraction
    println!("{}", use_func(10, 5, add));  // Output: 15
    println!("{}", use_func(10, 5, sub));  // Output: 5
}
