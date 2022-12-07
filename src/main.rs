use std::io;

/// This crate that shows how to get a  user input from the console

fn main() {
    //! # Main Function
    //! 
    //! ```
    //! fn main()
    //! ```
    //! 
    //! More Documentation Help: [Here]: https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html
    //! 
    //! This is a documentation comment for the main function
    let mut input = String::new();
    println!("Enter a number: ");
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You entered: {}", input);
        },
        Err(e) => println!("Error: {}", e),
    }

    // Printing Variables
    varPrint();

}

// Comments

// = Line Comment

/*  This
    IS
    A Multi-line Comment */

    /*
/// Documentation Comment
    */

    // Printing Variables

fn varPrint() {

    //Documenation Comment
    //! # Variable Printing Examples
    //! 
    //! ```
    //! fn varPrint()
    //! ```
    //! 
    //! ## Printing a Number
    //! Shows how to simply print a number
    //! 
    //! ## Printing a String
    //! Shows how to simply print a string
    //! 
    //! ## Printing with Multiple Variables
    //! Shows how to print multiple variables
    //! 
    //! ## Printing with Expressions
    //! Shows how to print expressions
    //! 
    //! ## Printing with Positional Arguments
    //! Shows how to print with positional arguments
    //! 
    //! ## Printing with Named Arguments
    //! Shows how to print with named arguments
    //! 
    //! ## Printing with Debug Trait
    //! Shows how to print with the debug trait
    //! 
    //! ## Printing with Traits
    //! Shows how to print with traits
    //! 
    //! ## Printing with Arrays
    //! Shows how to print arrays
    //! 
    //! ## Printing with Tuples
    //! Shows how to print tuples
    //! 
    //! ## Printing with Structures
    //! Shows how to print structures
    //! 
    //! ## Printing with Enums
    //! Shows how to print enums
    //! 
    //! ## Printing with Option
    //! Shows how to print options
    //! 
    //! ## Printing with Result
    //! Shows how to print results
    //! 
    //! More Documentation Help: [Here]: https://doc.rust-lang.org/book/ch03-02-data-types.html
    //! 

    // Printing a Number
    let x = 5;
    println!("The value of x is: {}", x);

    // Printing a String
    let s = String::from("Hello, World!");
    println!("{}", s);

    // Printing with Multiple Variables
    let x = 5;
    let y = 10;
    println!("x = {} and y = {}", x, y);

    // Printing with Expressions
    let x = 5;
    let y = 10;
    println!("x + y = {}", x + y);

    // Printing with Positional Arguments
    let x = 5;
    let y = 10;
    println!("{0} + {1} = {2}", x, y, x + y);

    // Printing with Named Arguments
    let x = 5;
    let y = 10;
    println!("{x} + {y} = {z}", x = x, y = y, z = x + y);

    // Printing with Debug Trait
    let s = String::from("Hello, World!");
    println!("{:?}", s);

    // Printing with Traits
    println!("binary: {:b}, hex: {:x}, octal: {:o}", 10, 10, 10);

    // Printing with Arrays
    let arr = [1, 2, 3, 4, 5];
    println!("arr = {:?}", arr);

    // Printing with Tuples
    let tup = (1, 2, 3, 4, 5);
    println!("tup = {:?}", tup);

    // Printing with Structures
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 1, y: 2 };
    println!("p = {:?}", p);

    // Printing with Enums
    enum Color {
        Red,
        Green,
        Blue,
    }
    let c = Color::Red;
    println!("c = {:?}", c);

    // Printing with Option
    let x: Option<i32> = Some(5);
    println!("x = {:?}", x);

    // Printing with Result
    let x: Result<i32, &str> = Ok(-3);
    println!("x = {:?}", x);

    


}