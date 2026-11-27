mod calculator {
    pub fn add(a: i32, b: i32) -> i32 {
        internal_helper(a, b)
    }

    fn internal_helper(a: i32, b: i32) -> i32 {
        a + b
    }
}

mod outer {
    pub mod inner {
        pub fn hi() {
            println!("hi");
        }
    }
}

use calculator::add;
use outer::inner::hi;
fn main() {
    let sum = add(1, 2);
    println!("{sum}");
    // internal_helper(1, 2); // not accessible
    hi();
}
