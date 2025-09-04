extern crate core;

use crate::exp::checkerboard::CheckerboardTest;

pub mod geom;
mod solver;
mod exp;
pub mod sort;
// target/release/GeomSortApp

fn main() {
    println!("Test App");
    let test_0 = CheckerboardTest::new(4000);
    test_0.run_all();
    // test_0.run_id_segments();
}
