// Usage of Module in rust

mod pizza_order {
    // Define a struct representing a Pizza
    pub struct Pizza {
        pub size: String,
        pub cheese: String,
        pub toppings: Vec<String>,
    }

    impl Pizza {
        // Define a method to create a new Pizza for lunch
        pub fn lunch(toppings: Vec<String>) -> Pizza {
            Pizza {
                size: String::from("large"),
                cheese: String::from("mozzarella"),
                toppings: toppings.to_vec(),
            } 
        } 
    }

    // Define a submodule to help customers with their orders
    pub mod help_customer {
        // Internal helper function
        fn help() {
            println!("Welcome to our pizza shop");
        }

        // Internal helper function
        fn seat_at_table() {
            println!("Please have a seat at the table"); 
        }

        // Public function to take customer's order
        pub fn take_order() {
            seat_at_table(); // Call internal function
            // Create a Pizza instance using the lunch method
            let cust_pizza: super::Pizza = super::Pizza::lunch(vec![
                String::from("mushrooms"), 
                String::from("green peppers")
            ]);
            // Serve the customer with the ordered pizza
            serve_customer(cust_pizza);
        }

        // Internal function to serve the customer
        fn serve_customer(cust_pizza: super::Pizza) {
            println!("Here is your pizza with toppings: {:?}", cust_pizza.toppings);
        }
    }
}

// Function to initiate the order process
fn order_food() {
   // Call the public function from the help_customer submodule
   crate::tutorial::new_9::pizza_order::help_customer::take_order();
}

pub fn my_module() {
   // Creates: Modules that produce a library or executable
   // Modules: Organize and handle privacy 
   // Packages: Build, test, and share crates 
   // paths: A way of naming an item, such as a struct, function, or module 

    // Call the function to start the food ordering process
    order_food();
}
