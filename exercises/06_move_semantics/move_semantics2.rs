// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

<<<<<<< HEAD:exercises/move_semantics/move_semantics2.rs
    let mut vec1 = fill_vec(vec0.clone());
=======
    let vec1 = fill_vec(vec0);
>>>>>>> 67a15ef27f73f83d4a29b955e970846fe42026fc:exercises/06_move_semantics/move_semantics2.rs

    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}
