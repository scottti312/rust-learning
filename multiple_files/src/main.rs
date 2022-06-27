pub mod garden;
pub use crate::garden::{operations, Options};

fn main() {
    let added = garden::operations::add_one(5);
    let subtracted = garden::operations::subtract_one(23);
    println!("{}, {}", added, subtracted);
    let me = Options {
        this: String::from("whatever"),
        test: 23,
    };
    println!("{}, {}", me.this, me.test);
}
