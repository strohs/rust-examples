mod back_of_house {

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    #[derive(Debug)]
    struct Lunch {
        entree: String,
        drink: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub mod cleaning {
        // have access to the private struct, Lunch, in the parent module above
        use crate::back_of_house::Lunch;

        // had to bring the Breakfast into scope, even though cleaning is a child mod

        pub fn print_lunch() {
            let lunch = Lunch { entree: "ribs".to_string(), drink: "tea".to_string() };
            println!("cleaning crew is having {:?}", lunch);
        }
    }
}

fn main() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    let scleaning = back_of_house::cleaning::print_lunch;

    scleaning();

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("{:?} {:?}", order1, order2);
}