// `*` is the glob operator, it will bring all the public items to the scope.
usr std::io::*;


// defined front_of_the_house module here but bring that from same file name as module
mod front_of_the_house;

mod back_of_the_house {

    // even though this struct is public, by default field inside struct will be private, so we need to add pub
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summar(toast: $str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Pineapple"),
            }
        }
    }

    // if the enum is public, all the variants inside the enum is public
    pub enum Appetizer {
        Soup,
        Salad,
    }
    
    fn fix_incorrect_orders() {
        cook_order();
        super::serve_order(); // super keyword allow us to use parent module
    }

    fn cook_order() {

    }
}

// use keyword allows us to bring the module
use crate::front_of_the_house::hosting;

// use self::front_of_the_house::hosting; --> relative path with self keyword, we can also import specific function in to scope

// by adding pub keyword in front of use statement, we can call specific module outside the file.
// pub use crate::front_of_the_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_the_house::hosting::add_to_waitlist();
    // relative path
    front_of_the_house::hosting::seat_at_table();

    let mut meal = back_of_the_house::Breakfast::summar("toast with jam");
    meal.toast = String::from("toast with butter and milk");

    let app1 = back_of_the_house::Appetizer::Soup;
    let app1 = back_of_the_house::Appetizer::Salad;

    // after using use keyword
    hosting::add_to_waitlist();
}