use rand::Rng;

fn main() {
    // io_example();
    // string_to_number_parse();
    // generate_random_number();
    conditional_block();
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

fn generate_random_number() {
    // Decide the range where the generated number will be between
    let range = 1..=1412; // Here we say that the range starts from 1 to 1412 included
                          // Generate the number
    let random_number = rand::thread_rng().gen_range(range);
    // Print the random number
    println!("The random number is {}", random_number);
}

fn conditional_block() {
    let user_age = 18;
    let age_to_vote = 18;

    println!("The user is {} years old", user_age);

    if user_age >= age_to_vote {
        println!("The user can vote!");
    } else {
        println!("The user cannot vote!");
    }

    // Ternary example
    let can_user_vote = if user_age >= age_to_vote { true } else { false };

    println!("Can user vote? {}", can_user_vote);
}
