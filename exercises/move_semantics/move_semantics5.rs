// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}


// The code is attempting to
// mutate a mutable reference to x (y) and
// then another mutable reference to x (z).
// This will result in a compilation error because
// Rust enforces the rule that you can't have
// more than one mutable reference to the same data at the same time.
// This rule ensures memory safety and prevents data races.
