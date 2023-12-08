#[derive(Debug)]
struct NumWithLogs {
    num: i32,
    logs: Vec<Vec<String>>,
}

fn square(num: i32) -> NumWithLogs {
    return NumWithLogs {
        num: num * num,
        logs: vec![vec![format!("Squared {} to get {}", num, num * num)]],
    };
}

fn add_one(mut num: NumWithLogs) -> NumWithLogs {
    num.num = num.num + 1;
    num.logs.push(vec![format!(
        "Added 1 to {} to get {}",
        num.num,
        num.num + 1
    )]);

    return num;
}

fn main() {
    println!(
        "square fn {:#?}, add one fn {:#?}",
        square(4),
        add_one(square(2))
    );

    println!("add one from square {:#?}", add_one(square(4)));
}
