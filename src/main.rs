#![allow(unused)] // #! is a directive that tells the compiler to ignore a warning

// Import the io library from the standard library
use std::io; // (::) is used to bring a module into scope
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;


// Import files from the other directory
mod other {
    pub mod new_1;
    pub mod new_2;
    pub mod new_3;
    pub mod new_4;
    pub mod new_5;
    pub mod new_6;
    pub mod new_7;
    pub mod new_8;
}

fn main() {
    // other::new_1::basic();
    // other::new_2::condition(); 
    // other::new_2::my_match();
    // other::new_3::array_loop();
    // other::new_4::my_string();
    other::new_4::advance_string();
    // other::new_7::tuple();
    // other::new_7::my_enums();
    // other::new_7::my_vectors();
    // other::new_5::my_function(); 
    // other::new_5::my_generics();
    // other::new_6::my_ownership();
    // other::new_7::my_hashmap();
    other::new_8::my_struct();
    other::new_8::trait_with_struct();
}

