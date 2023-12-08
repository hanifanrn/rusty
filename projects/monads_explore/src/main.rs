#[derive(Clone, Debug)]
struct NumWithLogs {
    num: i32,
    logs: Vec<String>,
}

fn wrap_with_logs(num: i32) -> NumWithLogs {
    return NumWithLogs { num, logs: vec![] };
}

fn square(num: NumWithLogs) -> NumWithLogs {
    let mut num_result = num.clone();
    num_result.num = num.num * num.num;
    num_result
        .logs
        .push(format!("Squared {} to get {}", num.num, num_result.num));

    return num_result;
}

fn add_one(num: NumWithLogs) -> NumWithLogs {
    let mut num_result = num.clone();
    num_result.num = num.num + 1;
    num_result
        .logs
        .push(format!("Added 1 to {} to get {}", num.num, num_result.num));

    return num_result;
}

fn main() {
    println!(
        "wrap with logs {:#?} square fn {:#?}, add one fn {:#?}",
        wrap_with_logs(4),
        square(wrap_with_logs(4)),
        add_one(square(wrap_with_logs(2)))
    );

    println!(
        "add one from square {:#?}",
        add_one(square(wrap_with_logs(3)))
    );

    println!(
        "square a square function {:#?}",
        square(square(wrap_with_logs(5)))
    );
}
