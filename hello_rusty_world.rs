fn main() {
    let mut hello = String::from("Hello, Rusty Wrold");

    let mut hello1 = add_exclamation(&mut hello);

    println!("{} is `{}`", "hello", hello);

    hello1.push_str("!");

    println!("{} is `{}`", "hello1", hello1);
}

fn add_exclamation(s: &mut String) -> String {
    let str = s;
    str.push_str("!");
    str.to_string()
}
