#![allow(unused)] // #! is a directive that tells the compiler to ignore a warning

// Import the io library from the standard library
use std::io; // (::) is used to bring a module into scope
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;


// Import files from the other directory
mod tutorial {
    pub mod new_1;
    pub mod new_2;
    pub mod new_3;
    pub mod new_4;
    pub mod new_5;
    pub mod new_6;
    pub mod new_7;
    pub mod new_8;
    pub mod new_9;
    pub mod new_10;
    pub mod new_11;
    pub mod new_12;
    pub mod new_13;
    pub mod new_14;
    pub mod new_15;
}

// new_9 to show how to use modules
use crate::tutorial::new_9::my_module;

fn main() {
    // tutorial::new_1::basic();
    // tutorial::new_2::condition(); 
    // tutorial::new_2::my_match();
    // tutorial::new_3::array_loop();
    // tutorial::new_4::my_string();
    // tutorial::new_4::advance_string();
    // tutorial::new_7::tuple();
    // tutorial::new_7::my_enums();
    // tutorial::new_7::my_vectors();
    // tutorial::new_5::my_function(); 
    // tutorial::new_5::my_generics();
    // tutorial::new_6::my_ownership();
    // tutorial::new_7::my_hashmap();
    // tutorial::new_8::my_struct();
    // tutorial::new_8::trait_with_struct();
    // my_module();
    // tutorial::new_10::my_error();
    // tutorial::new_11::my_iterator();
    // tutorial::new_11::your_closure();
    // tutorial::new_12::my_box();
    //  tutorial::new_13::my_thread();
    // tutorial::new_14::my_declarative_macro();
    tutorial::new_15::my_lifetime();
}
