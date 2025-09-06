extern crate core;

use crate::exp::checkerboard::CheckerboardTest;
use crate::exp::circle::CircleTest;
use crate::exp::random::RandomTest;

pub mod geom;
mod solver;
mod exp;
pub mod sort;
// target/release/rust_app --multithreading false --complex true --test 0

fn main() {
    println!("Test App");
    let test_0 = CheckerboardTest::new(2000);
    test_0.run_all();
    // test_0.run_custom();
    //
    let test_1 = RandomTest::new(10_000_000);
    test_1.run_all();

    let test_2 = CircleTest::new(10_000_000);
    test_2.run_all();
}
