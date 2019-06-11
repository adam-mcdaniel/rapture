use std::io::{stdin, stdout, Write};


pub fn input<S: ToString>(prompt: S) -> String {
    let mut buf = String::new();
    print!("{}", prompt.to_string());
    let _ = stdout().flush();

    stdin().read_line(&mut buf).expect("Could not get user input");

    if let Some('\n') = buf.chars().next_back() {
        buf.pop();
    }

    if let Some('\r') = buf.chars().next_back() {
        buf.pop();
    }

    return buf;
}