// DIAGNOSTIC: predict whether this program compiles before running its test.
// This file is deliberately outside Cargo's normal target discovery.
fn consume(label: String) -> usize {
    label.len()
}

fn main() {
    let label = String::from("engaged");
    let length = consume(label);
    println!("{label} has {length} bytes");
}
