use std::mem::size_of;
use std::io; // (::) is used to bring a module into scope
// import other libraries
use rand::Rng; // import the Rng trait from the rand crate (external library)

pub fn basic() {
    // taking input from user
    println!("What is your name?");
    let mut name:String = String::new();
    let greeting:&str = "Welcome to the Rust programming language!";

    // greeting = "Hello"; // this will throw an error because greeting is immutable
    name = "Ahmed ".to_string(); // String is a growable, UTF-8 encoded text

    io::stdin().read_line(&mut name ).expect("Failed to read line");

    println!("hello {} {}", name.trim_end(), greeting.trim()); // ! is a macro that prints to the console

    // different data type
    const ONE_MILE:u32 = 1_000_000; // unsigned 32-bit integer
    const PI:f32 = 3.14159; // 32-bit floating point number
    const _UNNAMED:&str = ""; // ( _ ) is used to ignore a unused variable

    let age:&str = "25"; // string slice
    let mut age:u32 = age.trim().parse().expect("Please type a number!"); // shadowing the age variable 
    age +=1;

    println!("You are {} years old & {}", age, ONE_MILE); 

    println!("MAX u32: {}", std::u32::MAX); 
    println!("MAX i32: {}", std::i32::MAX);
    println!("MAX f64: {}", std::f64::MAX);

    let is_true:bool = true; // boolean
    let my_char:char = 'A'; // character


    // math 
    let mut num_1:u32 = 10; // mutable 32-bit unsigned integer will allow us to change the value
    let num_2:u32 = 3;
    num_1 += 1; // increment
    println!("sum of {} + {} = {}", num_1, num_2, num_1 + num_2);

    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random number: {}", random_num);

    println!("size of isize {}", size_of::<&isize>());
    println!("size of str {}", size_of::<&str>());
}

