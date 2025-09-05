extern crate core;

use crate::exp::checkerboard::CheckerboardTest;

pub mod geom;
mod solver;
mod exp;
pub mod sort;
// target/release/rust_app --multithreading false --complex true --test 0

fn main() {
    println!("Test App");
    let test_0 = CheckerboardTest::new(9);
    test_0.run_all();
}
