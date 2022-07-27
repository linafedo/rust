mod front_of_house;
pub use crate::front_of_house::front_of_house::hosting as hos;

pub fn eat_at_restaurant() {
    hos::add_to_wait_list();
}
