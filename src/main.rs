use std::io;

/// This crate that shows how to get a  user input from the console

fn main() {
    //! # Main Function
    //! 
    //! ```
    //! fn main() {
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
}

// Comments

// = Line Comment

/*  This
    IS
    A Multi-line Comment */

    /*
/// Documentation Comment
    */