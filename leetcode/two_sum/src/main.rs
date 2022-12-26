use::std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();

    map.insert(1, "adri");
    map.insert(2, "hanifan");
    map.insert(3, "rusdi");
    
    let anak_ketiga: i32 = 3;
    println!("{}: anak ketiga", map.get(&anak_ketiga).unwrap());
}
