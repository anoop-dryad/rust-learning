fn main() {
    // The canonical use: same name, but the value transforms into a new TYPE.
    // Reading raw input as text, then turning it into the number you actually need.
    let input = "42"; // input is a &str (string)
    let input: i32 = input // input is now an i32 (number)
        .trim()
        .parse()
        .expect("not a valid number");

    println!("input + 1 = {}", input + 1); // 43 — arithmetic only works because it's now i32

    // Why shadowing and not `mut`? Because the TYPE changed (&str -> i32),
    // and `mut` can never change a type. Try uncommenting to see the compiler reject it:
    //
    // let mut bad = "42";
    // bad = bad.parse().unwrap(); // ERROR: expected `&str`, found `i32`

    // The scope demo, kept as a second proof that each `let` is a NEW variable:
    let x = 5;
    let x = x + 1; // new variable, 6
    {
        let x = x * 2; // new variable, 12 — inner scope only
        println!("inner x = {x}"); // 12
    }
    println!("outer x = {x}"); // 6 — inner one dropped at `}`
}
