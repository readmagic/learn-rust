mod front_of_house;

use front_of_house::hosting;
pub fn eat_at_restaurant() {
    //绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    //相对路径
    hosting::add_to_waitlist();

    let mut meal = front_of_house::serving::back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    //默认都是私有的
    //meal.seasonal_fruit = String::from("blueberries");
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
