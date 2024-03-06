// Function to print the sum of two integers
fn get_sum(x: i32, y: i32) {
    println!("{} + {} : {}", x, y, x + y);
}

// Function to calculate the sum and difference of two integers and return them as a tuple
fn get_sum_2(x: i32, y: i32) -> (i32, i32) {
    // Return a tuple containing the sum and difference of x and y
    return (x + y, x - y);
}

// Function to calculate the sum of elements in a vector
fn sum_list(list: &Vec<i32>) -> i32 {
    let mut sum: i32 = 0;

    // Iterate over each element in the vector and add it to the sum
    for &val in list.iter() {
        sum += &val; // Dereference val to access its value
    }
    return sum; // Return the sum of the elements
}

// Main function to demonstrate the usage of the above functions
fn my_function() {
    // Call get_sum function to print the sum of two integers
    get_sum(12, 4);

    // Call get_sum_2 function to calculate and print the sum and difference of two integers
    let (val_1, val_2) = get_sum_2(2, 5);
    println!("Nums : {} {}", val_1, val_2);

    // Create a vector of integers
    let num_list: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Call sum_list function to calculate and print the sum of elements in the vector
    println!("Sum of List: {}", sum_list(&num_list));
}

// Define a function `get_sum_gen` that takes two generic parameters `T`
// The `Add` trait bound ensures that the generic type `T` supports addition
// The `Output = T` associated type constraint specifies that the output type of the addition operation should be of type `T`
fn get_sum_gen<T: Add<Output = T>>(x: T, y: T) -> T {
    // Perform addition of the two generic values and return the result
    return x + y;
}

// Define a function `my_generics` to demonstrate the usage of generics
fn my_generics() {
    // Call `get_sum_gen` with integer arguments and print the result
    println!("Get sum using generics: {}", get_sum_gen(6, 11));

    // Call `get_sum_gen` with floating-point arguments and print the result
    // Note: The format specifier {:.2} will round the result to two decimal places
    println!("Get sum using generics: {:.2}", get_sum_gen(6.23, 11.4));
}
