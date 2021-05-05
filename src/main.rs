fn main() {
    // fATAL ERRORS WITH PANIC!

    // Panicking is the simplest error handling mechanism in Rust.
    // We can use the panic! macro to panic the current thread. It prints and error message, unwinds and cleans up the stack, and then exists the program.

    panic!("Farewell!");
    // The above code would exit with status code 101 and print the following message:
    // thread 'main' panicked at 'Farewell!', src/main.rs:7:5

    // In general terms, we should use panic! when a program reaches an unrecoverable state meaning anything where there is absolutely no way to recover from the error.

    // Rust panics on some operations such as a division by zero or an attempt to access an index that isn't present in an array, a vector, or a hash map.

    let v = vec![0, 1, 2, 3];
    println!("{}", v[6]); // this will cause a panic!
}
