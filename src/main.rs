fn main() {
    // io_example();
    string_to_number_parse();
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

fn string_to_number_parse() {
    // Variable shadowing in Rust is permitted. In fact here there are two variable with the same name
    // but with different types.
    let number = "1412";
    let number: u32 = number.parse().expect("String was not a number");
    // Print the converted variable
    println!("{}", number);
}
