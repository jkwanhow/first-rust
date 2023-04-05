use std::env;
use std::fs;
use std::collections::HashMap;

// plan is to generate a huffman tree
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut occurance_of_letter= HashMap::new();

    let file_path = &args[1];

    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    for c in contents.chars() {
        // get current value and increment by 1 or else insert 1.
        occurance_of_letter.entry(c).and_modify(|counter| *counter += 1).or_insert(1);
        let current_value = *occurance_of_letter.get_mut(&c).unwrap();
        println!("{} {}", c, current_value);
    }

    // plan next is to iterate letters then inplace add them into a tree or an array
    println!("With text:\n{contents}");
    
}
