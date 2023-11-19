use std::io;

fn main() {
    let my_string: String = String::from("   my   string   ");
    println!("{}", trimmed_space(&my_string[..]));

    let my_string_literal: &str = "my  string literal    ";
    println!("{}", trimmed_space(my_string_literal));

    let mut input_string: String = String::new();

    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");

    println!("{}", trimmed_space(&input_string[..]));
}

fn trimmed_space(str: &str) -> &str {
    let chars = str.chars();

    let mut first_non_space: usize = str.len();
    let mut last_non_space: usize = 0;

    for (i, ch) in chars.enumerate() {
        if ch != ' ' {
            if first_non_space == str.len() {
                first_non_space = i;
            }
        }
        last_non_space = i;
    }

    if last_non_space == 0 {
        &str[first_non_space..]
    } else {
        &str[first_non_space..last_non_space]
    }
}
