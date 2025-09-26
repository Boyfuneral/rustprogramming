
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0 // Correct guess
    } else if guess > secret {
        1 
    } else {
        -1 
    }
}

fn main() {
    println!("\n--- Assignment 3: Guessing Game ---");
    
  
    let secret_number: i32 = 42; 
    let mut guess_count: i32 = 0;
    
    println!("I've picked a secret number. Try to guess it!");

    
    let guesses: [i32; 5] = [50, 25, 35, 45, 42]; 
    let mut guess_index: usize = 0;

   
    loop {
        if guess_index >= guesses.len() {
            println!("Simulated guesses ran out!");
            break;
        }

        let guess: i32 = guesses[guess_index];
        guess_count += 1;
        
        println!("\nGuess #{}: {}", guess_count, guess);
        
        
        let result = check_guess(guess, secret_number);

    
        if result == 0 {
            println!(" You guessed correctly! The number was {}!", secret_number);
            break; 
        } else if result == 1 {
            println!("Your guess is too high.");
        } else {
            println!("Your guess is too low.");
        }
        
        guess_index += 1;
    }
    
    
    println!("\nIt took you {} guesses to find the secret number.", guess_count);
}