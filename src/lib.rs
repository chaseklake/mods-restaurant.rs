pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// Defining a restaurant library to demonstrate Modules
// Creates a new private module "front_of_house"
mod front_of_house {
    // The first submodule "front_of_house::hosting"
    // "pub" here means anything with access to "front_of_house" can access "hosting"
    pub mod hosting {
        // Some functions of "hosting" module
        // "pub" here means that anything with access to "hosting" can access this function
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    // The second submodule "front_of_house::serving"
    // This is a "sister" module to "hosting"
    mod serving {
        // Some functions
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// An exposed, or "public" function that a library user can call
// Since this function is in the same module as "front_of_house," it can access "front_of_house"
pub fn eat_at_restaurant() {
    // Referencing "add_to_waitlist()" though its Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Referencing "add_to_waitlist()" though its Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast:
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about the toast type:
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please.", meal.toast);

    // This won't compile because "seasonal_fruit" is private:
    // meal.seasonal_fruit = String::from("blueberries");
}

// How to use a "use" to create a shorthand for a submodule:
use crate::front_of_house::hosting;
// NOTE: if we add "pub" to the front of this, it "re-exports" the "hosting" submodule.
// This means any external code can use "restaurant::hosting" instead of "restaurant::front_of_house::hosting"
// This allows us to expose a different structure to our users.

pub fn eat_at_restaurant2() {
    // Using the shorthand to reference the function, now that we've set a "use":
    hosting::add_to_waitlist();

    // NOTE: We could bring the function directly into scope as follows:
    // "use crate::front_of_house::hosting::add_to_waitlist()""
    
    // Then added the function directly:
    // "add_to_waitlist();"

    // The reason we don't do this for functions is clarity: 
    // Adding "hosting::" makes it clearer where the function is defined.
}

mod customer {
    pub fn eat_at_restaurant() {
        // The following won't compile since the "use" from before does not exist in "mod customer"
        // hosting::add_to_waitlist();
        // To fix this, we could add "super::hosting" before this function to bring "hosting" back into scope
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        // cook_order() is in the same scope as fix_incorrect_order()
        cook_order();
        // An example of using "super" to access a function in the parent's (back_of_house) scope
        super::deliver_order();
    }

    fn cook_order() {}

    // Struct fields are private by default:
    pub struct Breakfast {
        pub toast: String, // Defined by the customer's order, thus public
        seasonal_fruit: String, // Defined by the chef, thus not public
    }

    // This constructor is needed bc "Breakfast" has a private field that must be defined upon instancing
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast), 
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // Making an enum public makes all of its variants public by default:
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// Some notes about "use":


// 1) Both of these have a "Result" object, but because their "Results" are not directly brought into scope this is fine.
// use std::fmt;
// use std::io;

// The following would cause an error:
// use std::fmt::Result;
// use std::io::Result;
// fn function() -> Result {}; // Is this a "io" or "fmt" result?? ERROR!

// We can get around this with the "as" keyword, letting us define similar names with new names:
// use std::fmt::Result;
// use std::io::Result as IoResult; // totally fine!

// This means a "fmt" Result:
// fn function1() -> Result { // Do stuff }

// This means an "io" Result:
// fn function2() -> IoResult<()> { // Do stuff }


// 2) We can nest paths to bring multiple objects from the same library into scope:

// use std::cmp::Ordering;
// use std::io;
// ...is the same as:
// use std::{cmp::Ordering, io};

// To bring in two paths where one is the base path, we use "self":
// use std::io;
// use std::io::Write;
// ...is the same as:
// use std::io::{self, Write};


// 3) We can bring ALL public items in a path into scope with the Global (or "Glob") operator, "*":

// use std::collections::*;

// This can make it unclear what things are and aren't in scope.
// Usually this is done for testing or as part of the prelude pattern.