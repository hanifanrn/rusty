fn main() {
    let my_num: usize = adding_one(4);

    println!("{}", my_num);
}

fn adding_one(x: usize) -> usize {
    x + 1
} 