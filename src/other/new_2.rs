use std::cmp::Ordering; 

pub fn condition() {
    // conditonal statement
    let age:i8 = 122;
    if (age >= 1) && (age <= 18) {
        println!("Your age under 18");
    } else if (age >= 18) && (age <= 80 ){
        println!("Cong Your 18+"); 
    } else {
        println!("Go die");
    }

    // tenary operator
    let mut my_age:i32 = 22;
    let can_vote:bool = if my_age >= 18 && my_age <= 80 {
        true
    } else {
        false
    };
    println!("Can vote: {}", can_vote);

}

pub fn my_match(){
    /*
     * The match expression in Rust allows you to compare a value against a series of patterns and execute code based on which pattern matches. 
     * It's similar to a switch or case statement found in other programming languages, but offers more flexibility and safety. 
     * match is widely used for pattern matching, exhaustive handling of all cases, and avoiding common programming errors like forgetting to handle certain cases. 
     * It's a powerful control flow construct that enables expressive and type-safe code in Rust.
     */

    // match are used to compare the value of a variable against a pattern 
    let age2: i32 = 22;
    match age2 {
        1..=18 => println!("Your age under 18"), 
        19..=80 => println!("Cong Your 18+"),
        81..=i32::MAX => println!("Fuck you"),
        _ => println!("Go die") // _ will match everything else
    };

    let age3: i32 = 18;
    let voting_age: i32 = 18;

    // here we compare age wih voting_age
    match age3.cmp(&voting_age) {
        Ordering::Less => println!("you can't vote"),
        Ordering::Greater => println!("can vote"),
        Ordering::Equal => println!("you gain the right to vote"),
    };
}
