// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

fn main() {
    let mut data = "Rust is great!".to_string();

    get_char(&data);

    // pattern 5: mutable borrow
    // if you don't want to return the value
    // you can make the variable mutable and pass a mutable value to it
    string_uppercase(&mut data);

    println!("{}", data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &mut String) {
    *data = data.to_uppercase();

    println!("{}", data);
}