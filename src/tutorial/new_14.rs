// Macros in Rust are powerful tools for metaprogramming
//  - They allow you to define custom syntax extensions
//  - They generate code at compile time (unlike functions that run at runtime)

// This line makes the `header` macro available outside the current module
#[macro_export]
macro_rules! header {
    // Macro with one literal argument
    ($val:literal) => {
        // Expands to a string literal with "# " prepended
        {
            concat!("# ", $val)
        }
    };

    // Macro with two literal arguments
    ($val:literal, $val2:literal) => {
        // Expands to a string literal with "# " prepended to each argument
        {
            concat!("# ", $val, "\n# ", $val2)
        }
    };

    // Macro with multiple literal arguments (at least one)
    // + any number but at least one
    // * any number of repetition
    // ? optional fragment with zero or one occurrence
    ($($val:literal),+) => {
        // Expands to a string literal with "# " prepended to each argument
        //  and joined with newlines. The trailing newline is trimmed.
        {
            concat!($("# ", $val, "\n",)+).trim_end()
        }
    };
}

// Module to group related code for the declarative macro example
mod declarative_macro {
    pub fn test_header() {
        // Array containing macro invocations with different arguments
        let content = [
            header!("Hello, world!"),
            header!("Bad, world!", "Sed, world!"),
            header!("Curl, world!", "Beautiful, world!", "Good, world!"),
        ].join("\n\n"); // Join elements with double newlines

        println!("{content}"); // Print the generated content
    }
}

// Function to call the test_header function from the declarative_macro module
pub fn my_declarative_macro() {
    declarative_macro::test_header();
}

