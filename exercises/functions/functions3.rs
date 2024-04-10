// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.

// call_me() is called without an argument, but it is defined with an argument.
// We can fix this by removing the argument from the function definition.
// Or just call_me(3) in main().

fn main() {
    call_me();
}

fn call_me() {
    let num = 3;
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
