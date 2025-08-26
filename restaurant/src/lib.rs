
mod front_of_house;
mod back_of_house;


use front_of_house::hosting;
use back_of_house::{ Breakfast, Appetizer };

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    let mut meal = Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // seasonal_fruit do not have pub, so this code is compile error
    // let br = Breakfast { toast: String::from("White"), seasonal_fruit: String::from("grape"), };
    
    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
}
