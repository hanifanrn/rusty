fn main() {
    println!("test unit testing");
}

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn it_doesnt_works() {
    assert_eq!(2 + 2, 5);
}
