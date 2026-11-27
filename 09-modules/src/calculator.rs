pub mod advanced;

pub fn add(a: i32, b: i32) -> i32 {
    internal_helper(a, b)
}

fn internal_helper(a: i32, b: i32) -> i32 {
    a + b
}
