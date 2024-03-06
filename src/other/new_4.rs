fn my_string(){
    // two types of string in rust
    // 1. String (growable, heap-allocated data structure) use vector to store UTF-8 encoded text
    // 2. &str (string slice, a reference(point) to a string)

    let mut st1:String = String::new(); // create a new empty string
    st1.push('A'); // push a character to the string
    st1.push_str("kib"); // push a string to the string

    for word in st1.split(" "){
        println!("{}", word);
    }

    let st2:String = st1.replace("kib", "nother");
    println!("{}", st2);

    let st3 = String::from("H E L L O H I"); // create a new string 
    let mut v1:Vec<char> = st3.chars().collect(); // convert the string to a vector of characters

    v1.sort(); // sort the vector
    v1.dedup(); // remove the duplicates from the vector

    // cycle through the vector and print the characters 
    for char in v1  {
        print!("{}", char); 
    }
    print!(" \n");

    let st4: &str =  "Hello, World!"; // string literal 
    let mut st5: String = st4.to_string(); // heap-allocated string
    println!("{}", st5);

    // convert a string to array of bytes 
    let byte_array = st5.as_bytes();
    println!("{:?}", byte_array);

    let st6: &str = &st5[0..4]; // slicing the string
    println!("{}", st6);
    println!("{}", st6.len());
    st5.clear(); // clear the string
                 //
    let st6 = String::from("Hello, World!");
    let st7 = String::from(" From Rust");
    let st8 = st6 + &st7; // concatenation and ownership transfer of st6 to st8

    for char in st8.bytes(){
        print!("{} ", char );
    }
    println!("");

    // value Casting
    let int_u8:u8 = 5;
    let int2_u8:u8 = 4;
    let int3_u32:u32 =(int_u8 as u32) + (int2_u8 as u32); // Convert the values of int_u8 and int2_u8 to u32 before performing addition using "as" keyword
    println!("casting value of int1 and int2 is: {}", int3_u32);
}

