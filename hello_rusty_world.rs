fn main() {
    let mut hello = String::from("Hello, Rusty Wrold");

    let mut hello1 = add_exclamation(&mut hello);

    println!("{} is `{}`", "hello", hello);

    hello1.push_str("!");

    println!("{} is `{}`", "hello1", hello1);

    my_name_function();
    my_last_name_function();
}

fn my_name_function() {
    let hanifan = String::from("Hanifan");
    println!("{hanifan}");
}

fn my_last_name_function() {
    let nurahman = String::from("Nurahman");
    println!("{}", nurahman);
}

fn add_exclamation(s: &mut String) -> String {
    let str = s;
    str.push_str("!");
    return str.to_string();
}
