fn main() {
    // 1. initialize
    let mut my_array = [0;5];
    
    // 2. get length
    let my_array_length = my_array.len();
    println!("The size of my_array is: {}", my_array_length);
    
    // 3. access element
    println!("The first element is: {}", my_array[0]);

    // 4. Iterate all Elements
    println!("[Version 1] The contents of all are:");
    for i in 0..my_array.len() {
        println!("{}", my_array[i]);
    }

    println!("[Version 2] The contents of my_array are:");
    for i in my_array {
        println!("{}", i);
    }

    // 5. Modify Element
    my_array[0] = 8;
    println!("modified array: {:?}", my_array);
    
    // 6. Sort
    my_array.sort();
    println!("sorted array: {:?}", my_array);
}
