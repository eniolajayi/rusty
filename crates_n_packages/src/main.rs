pub mod backyard;
use crate::backyard::garden::vegetable::Asparagus;

fn main() {
    let plant = Asparagus {};

    println!("I'm growing {plant:?}!");
}
