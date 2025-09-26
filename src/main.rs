// Problem #1: String Concatenation with Borrowing
fn concat_strings(s1: &String, s2: &String) -> String {
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}

// Problem #2: Clone and Modify
fn clone_and_modify(s: &String) -> String {
    let mut cloned_s = s.clone();
    cloned_s.push_str("World!");
    cloned_s
}

// Problem #3: Mutable Reference Sum
#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    for i in low..=high {
        *total += i;
    }
}


fn main() {
    println!("--- Running Problem #1 ---");
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result);
    println!(""); 

    println!("--- Running Problem #2 ---");
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s);
    println!("Modified: {}", modified);
    println!(""); 

    println!("--- Running Problem #3 ---");
    let mut total = 0;
    let low = 0;
    let high = 100;
    sum(&mut total, low, high);
    println!("Sum from {} to {} is: {}", low, high, total);
}