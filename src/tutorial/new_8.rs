use std::ops::{Add, Mul, Div}; // Allow us to perform arithmetic operations with generics

// Define a custom data structure called Customer
#[derive(Debug)]
struct Customer {
    name: String,
    address: String,
    balance: f32,
}

// Implementation of a function that demonstrates the usage of structs
pub fn my_struct() {
    // Create an instance of the Customer struct
    let mut akib: Customer = Customer {
        name: String::from("Akib"),
        address: String::from("mirpur, dhaka-1216"),
        balance: 1990000.25425,
    };
    // Modify the balance field of the Customer struct
    akib.balance = 2000.01;

    // Print the modified Customer struct
    println!("{:?}", akib);
}

// Define a trait called Shape, which serves as a blueprint for geometric shapes
trait Shape<T> {
    // Method to create a new shape instance
    fn new(length: T, width: T , radius: T) -> Self;

    // Method to calculate the area of the shape
    fn area(&self) -> T;
}

// Define a struct called Rectangle, implementing the Shape trait
#[derive(Debug)]
struct Rectangle<T> {
    length: T,
    width: T,
}

// Define a struct called Circle, implementing the Shape trait
#[derive(Debug)]
struct Circle<T> {
    radius: T,
}

// Implementation of the Shape trait for the Rectangle struct
impl<T> Shape<T> for Rectangle<T>
where
    T: Mul<Output = T> + Copy, // T must support multiplication and be copyable 
{
    // Method to create a new Rectangle instance
    fn new(length: T, width: T, radius: T) -> Rectangle<T> {
        Rectangle {
            length,
            width
        }
    }

    // Method to calculate the area of the Rectangle
    fn area(&self) -> T {
        self.length * self.width
    }
}

// Implementation of the Shape trait for the Circle struct
impl<T> Shape<T> for Circle<T>
where
    T: Mul<Output = T> + Div<Output = T> + Copy + From<f32>, // T must support division, multiplication, conversion from f32, and be copyable
{
    // Method to create a new Circle instance
    fn new(length: T, width: T, radius: T) -> Circle<T> {
        Circle { radius }
    }

    // Method to calculate the area of the Circle
    fn area(&self) -> T {
        let pi = T::from(3.1416);
        (self.radius * T::from(2.00)) * pi
    }
}

// Implementation of a function that demonstrates the usage of traits with structs
pub fn trait_with_struct() {
    // Create a Rectangle instance and calculate its area
    let rec: Rectangle<f32> = Shape::new(10.243, 12.243, 0.00);
    println!("Area Of Rectangle is: {:?}", rec.area());
    
    // Create a Circle instance and calculate its area
    let cir: Circle<f32> = Shape::new(0.00, 0.00, 5.00);
    println!("Area Of Circle is: {:?}", cir.area());
}


pub fn my_enums () {
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
