fn main() {
    io_example();
}

fn io_example() {
    // Ask to user to type something
    println!("Type your name: ");
    // Create new mutable variable
    let mut name = String::new();
    // Read the input from stdin
    std::io::stdin()
        .read_line(&mut name)
        .expect("String has not been inserted");
    // Print the given string
    println!("Hello {}", name);
}
