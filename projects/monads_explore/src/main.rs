fn square(num: i32) -> i32 {
    return num * num;
}

fn add_one(num: i32) -> i32 {
    return num + 1;
}

fn main() {
    println!("square fn {}, add one fn {}", square(4), add_one(1));

    println!("add one from square {}", add_one(square(4)));
}
