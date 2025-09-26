
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    println!("\n--- Assignment 2: Number Analyzer ---");
    
   
    let numbers: [i32; 10] = [1, 2, 3, 5, 10, 15, 18, 20, 30, 11];

    println!("\n--- Even/Odd and FizzBuzz Analysis (For Loop) ---");
    
    
    for number in numbers {
        
        if number % 3 == 0 && number % 5 == 0 {
            println!("{}: FizzBuzz", number);
        } else if number % 3 == 0 {
            println!("{}: Fizz", number);
        } else if number % 5 == 0 {
            println!("{}: Buzz", number);
        } else if is_even(number) {
            println!("{}: Even", number);
        } else {
            println!("{}: Odd", number);
        }
    }

    println!("\n--- Sum of Numbers (While Loop) ---");
    
   
    let mut sum: i32 = 0;
    let mut index: usize = 0;
    
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    
    println!("The sum of all numbers is: {}", sum);
    
    println!("\n--- Largest Number (Loop) ---");
    
   
    let mut largest: i32 = numbers[0]; 
    let mut current_index: usize = 1;
    
    loop {
        if current_index >= numbers.len() {
            break; 
        }
        
        if numbers[current_index] > largest {
            largest = numbers[current_index];
        }
        
        current_index += 1;
    }
    
    println!("The largest number in the array is: {}", largest);
}