#[derive(Clone, Debug)]
struct NumWithLogs {
    num: i32,
    logs: Vec<String>,
}

fn wrap_with_logs(num: i32) -> NumWithLogs {
    return NumWithLogs { num, logs: vec![] };
}

fn run_with_logs(arg: NumWithLogs, func: fn(i32) -> NumWithLogs) -> NumWithLogs {
    let new_number_with_logs = func(arg.num);
    let num = new_number_with_logs.num;
    let mut logs = arg.logs;
    logs.push(new_number_with_logs.logs[0].clone());

    return NumWithLogs { num, logs };
}

fn square(num: i32) -> NumWithLogs {
    let num_result = num * num;
    let logs = vec![format!("Squared {} to get {}", num, num_result)];

    return NumWithLogs {
        num: num_result,
        logs,
    };
}

fn add_one(num: i32) -> NumWithLogs {
    let num_result = num + 1;
    let logs = vec![format!("Added 1 to {} to get {}", num, num_result)];

    return NumWithLogs {
        num: num_result,
        logs,
    };
}

fn main() {
    let num = 3;
    let num_a = wrap_with_logs(num);
    println!("wrap {:#?}", &num_a);
    let num_b = run_with_logs(num_a, add_one);
    println!("add 1 {:#?}", &num_b);
    let num_c = run_with_logs(num_b, square);
    println!("square from add 1 {:#?}", num_c);
    let num_d = run_with_logs(num_c, add_one);
    println!("final result and log {:#?}", num_d);
}
