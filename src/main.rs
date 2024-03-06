#![allow(unused)] // #! is a directive that tells the compiler to ignore a warning

// Import the io library from the standard library
use std::io; // (::) is used to bring a module into scope
use std::mem::size_of;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering; 
use std::ops::Add; // allow us to perform addition with generics

// import other libraries
use rand::Rng; // import the Rng trait from the rand crate (external library)

// importing file
include!("../src/other/new_1.rs");
include!("../src/other/new_2.rs");
include!("../src/other/new_3.rs");
include!("../src/other/new_4.rs");
include!("../src/other/new_5.rs");
include!("../src/other/new_6.rs");
include!("../src/other/new_7.rs");

fn main() {
    // basic();
    // condition(); 
    // my_match();
    // array_loop();
    // tuple();
    // my_string();
    // my_enums();
    // my_vectors();
    // my_function(); 
    // my_generics();

    my_ownership();
}

