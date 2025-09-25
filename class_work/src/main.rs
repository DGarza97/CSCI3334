fn concat_strings(s1: &String, s2: &String) -> String {
    let word1 = s1.to_string();
    let word2 = s2.to_string();
    let result = word1 + &word2;
    result
}

fn clone_and_modify(s: &String) -> String {
    let mut cloned = s.clone();
    cloned.push_str("World!");
    cloned
}

fn sum(total: &mut i32, low: i32, high: i32) {
    *total = 0;
    let mut current = low;
    while current <= high {
        *total += current;
        current += 1;
    }
    0;
}




fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"

    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"

    let mut total = 0; 
    sum(&mut total, 0, 100); 
    println!("Total sum from 0 to 100 is: {}", total); 
}
