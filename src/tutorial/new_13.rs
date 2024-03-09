/* 
common problem with parallel programming involves:
    1. Thread are accessing data in wrong order
    2. Thread are blocked from executing because of contention
 
Over requirement to proceed with execution
*/
/*
Concurrency in programming refers to the ability of a system to execute multiple tasks or processes simultaneously. 
It's a fundamental concept in modern software development, enabling programs to perform multiple operations concurrently, 
improving performance and responsiveness.
*/

use std::thread; // For creating and managing threads
use std::time::Duration; // For pausing execution for a duration
use std::rc::Rc; // Reference counting for shared ownership (not used in this code)
use std::cell::RefCell; // Mutable memory cell with interior mutability (not used in this code)
use std::sync::{Arc, Mutex}; // Thread-safe smart pointers for shared mutable state

// Example of a simple thread
fn simple_thread() {
    // Spawn a new thread that prints numbers
    let thread1 = thread::spawn(|| {
        for i in 1..20 {
            println!("spawned {} ", i);
            thread::sleep(Duration::from_millis(1)); // Delay for 1 millisecond
        }
    });

    // Print numbers from the main thread
    for i in 1..10 {
        println!("main {} ", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for the spawned thread to finish
    thread1.join().unwrap();
}

// Bank struct to model a bank account
struct Bank {
    balance: f32,
}

// Function to withdraw money from a bank account (thread-safe)
fn withdraw(the_bank: &Arc<Mutex<Bank>>, amount: f32) {
    // Acquire a lock on the bank account to ensure exclusive access
    let mut bank_ref = the_bank.lock().unwrap();

    if bank_ref.balance >= amount {
        bank_ref.balance -= amount;
        println!("Withdrawal successful. Remaining balance: {}", bank_ref.balance);
    } else {
        println!("Insufficient funds. Withdrawal failed.");
    }
}

// Function to simulate a customer making a withdrawal
fn customer(the_bank: &Arc<Mutex<Bank>>, amount: f32) {
    // Arc: Atomic Reference Counting, allowing shared ownership of values across multiple threads.
    // Mutex: Provides mutual exclusion, ensuring that only one thread can access the Bank object at a time to prevent data races and ensure thread safety.
    withdraw(the_bank, amount);
}

// Main function demonstrating thread synchronization
pub fn my_thread() {
    // Create a shared, thread-safe bank account with an initial balance
    let bank = Arc::new(Mutex::new(Bank { balance: 1000.0 }));

    // Create 16 threads representing customers
    let handles = (0..16).map(|_| {
        let bank_ref = bank.clone(); // Share a reference to the bank account
        thread::spawn(move || { // Move closure to capture bank_ref
            customer(&bank_ref, 100.0); // Perform a withdrawal
        })
    });

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the final balance (ensure thread-safe access)
    println!("Final balance: {}", bank.lock().unwrap().balance);
}

