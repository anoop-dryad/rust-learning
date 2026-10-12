// ============================================================================
// Layer 8, Check 1 — Generics: write logic once, for any type
//
// Run: cargo run -p traits --bin generics
//
// <T> declares a TYPE PARAMETER. Generics = write-once-for-any-type, checked at
// COMPILE time, and ZERO-COST (monomorphization: the compiler stamps out a
// specialized copy per concrete type used, so no runtime dispatch overhead).
//
// Key tension: T is "any type", so you can only DO to it what you PROMISE via
// trait bounds. Bound T with the MINIMAL trait that covers what you actually do.
// ============================================================================

#[derive(Debug)]
#[allow(dead_code)]
struct Pair<T> {
    first: T,
    second: T,
}

fn main() {
    let ints: Vec<i32> = vec![10, 2, 3, 4, 5, 6];
    let strs: Vec<&str> = vec!["abc", "b"];
    let chars: Vec<char> = vec!['a', 'z', 'm'];
    let floats: Vec<f64> = vec![1.5, 3.5, 2.5]; // f64: PartialOrd but NOT Ord

    // --- first<T>: NO bound needed (just returns &list[0], doesn't operate on T) ---
    println!("first int : {}", first(&ints));
    println!("first str : {}", first(&strs));

    // --- largest (manual loop): needs only PartialOrd (what `>` requires) ---
    //     PartialOrd accepts floats; Ord would NOT.
    println!("largest int   : {:?}", largest(&ints));
    println!("largest char  : {:?}", largest(&chars));
    println!("largest float : {:?}", largest(&floats)); // works BECAUSE of PartialOrd

    // --- largest_using_iter: .max() requires Ord (total order) -> no floats here ---
    println!("largest (iter): {:?}", largest_using_iter(&ints));

    // --- sum: a generic needing THREE things about T, each for a real reason ---
    println!("5 + 10   = {}", sum(&5, &10));
    println!("1.5+2.5  = {}", sum(&1.5, &2.5));

    // --- generic struct over T ---
    let int_pair = Pair {
        first: 10,
        second: 20,
    };
    let str_pair = Pair {
        first: String::from("Anoop"),
        second: String::from("Manasa"),
    };
    println!("{int_pair:?}");
    println!("{str_pair:?}");
}

// No bound: doesn't compare or operate on the value, just hands back a reference.
fn first<T>(list: &[T]) -> &T {
    &list[0]
}

// PartialOrd = the trait `>` needs. Minimal correct bound -> accepts f64.
// (Using Ord here would silently REJECT Vec<f64>, because f64 isn't Ord — NaN.)
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// .max() on an iterator requires Ord (a TOTAL order). Correct to use Ord here.
fn largest_using_iter<T: Ord>(list: &[T]) -> Option<&T> {
    list.iter().max()
}

// `+` via the Add trait needs THREE things pinned down:
//   Add<Output = T> : two T's can be added AND the result is a T
//   + Copy          : so *a / *b COPY the value (can't MOVE out of a &ref — Layer 3)
// Multiple bounds are joined with `+`.
fn sum<T: std::ops::Add<Output = T> + Copy>(a: &T, b: &T) -> T {
    *a + *b
}
