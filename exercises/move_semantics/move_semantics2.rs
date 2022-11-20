// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

fn main() {
    let vec0 = Vec::new();

    // pattern 2: borrow instead of own, and return another array
    // here using borrow
    // note that if we are returning another Vec (instead of &Vec)
    // something needs to be done because they are different
    let mut vec1 = fill_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    // here we need to clone a new value!
    // but it might be expensive if it's a large vector
    let mut vec = vec.clone(); 

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
