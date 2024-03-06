fn tuple(){
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

fn my_vectors () {
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

fn my_enums () {
    /*
     * Enums in Rust provide a way to define a custom data type by enumerating its possible variants. 
     * Each variant can hold different types of data or no data at all. 
     * Enums are versatile and commonly used for representing and handling different states or options within a program. 
     * They offer pattern matching and associated data, allowing for expressive and type-safe code. 
     * Enums are a powerful tool for modeling domain-specific concepts and facilitating structured programming in Rust.
     */

    // In Rust, an enum (short for "enumeration") is a custom data type that allows you to define a type by enumerating its possible variants. Each variant of an enum can hold different types of data or no data at all.
    enum Day {
        Saturday,
        Sunday,
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday
    }

    // The impl block in Rust is used for implementing methods associated with a particular type. 
    impl Day {
        fn is_weekend(&self) -> bool {
            match self {
                Day::Saturday | Day::Sunday => true,
                _ => false
            }
        }
    }

    // Here, today is assigned the value Day::Sunday, indicating that today is Sunday.
    let today:Day = Day::Sunday;

    match today {
        Day::Saturday => println!("Weekend"),
        Day::Sunday => println!("Weekend"),
        Day::Monday => println!("Every one hate Monday"),
        Day::Tuesday => println!("Donut Day"),
        Day::Wednesday => println!("Hump Day"),
        Day::Thursday => println!("Pay Day"),
        Day::Tuesday => println!("Donut day"),
        Day::Friday => println!("Almost Weekend"),
    }

    println!("Is today the Weekend: {}", today.is_weekend());
}

