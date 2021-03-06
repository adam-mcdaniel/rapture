use std::io::{stdin, stdout, Write};

/// This function prompts the user with a message and returns the user's input.
/// It also pops off trailing carriage returns.
pub fn input<S: ToString>(prompt: S) -> String {
    let mut buf = String::new();
    print!("{}", prompt.to_string());
    let _ = stdout().flush();

    stdin().read_line(&mut buf).expect("Could not get user input");

    while let Some('\n') = buf.chars().next_back() {
        buf.pop();
    }

    while let Some('\r') = buf.chars().next_back() {
        buf.pop();
    }

    return buf;
}

/// Used to prompt the user with a yes or no question.
/// If they answer with Y or y, this function returns true.
pub fn yes_or_no<S: ToString>(prompt: S) -> bool {
    let response = input(prompt);

    response.to_lowercase().trim() == "y"
}