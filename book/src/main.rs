use std::fs::File;
use std::io::{self, Write, BufReader, BufRead};

// Derive Debug and Clone for easier printing and manipulation in main
#[derive(Debug, Clone)]
struct Book {
    title: String,
    author: String,
    year: u16,
}

/// Saves a vector of Book structs to a specified file.
/// Each book is saved on a new line with fields separated by commas.
fn save_books(books: &Vec<Book>, filename: &str) {
    // Attempt to create the file, handling potential errors
    let mut file = match File::create(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error creating file {}: {}", filename, e);
            return; // Exit function on failure
        }
    };

    for book in books {
        // Format the book data as a comma-separated string
        let line = format!("{},{},{}\n", book.title, book.author, book.year);
        
        // Write the line to the file, handling potential errors
        if let Err(e) = file.write_all(line.as_bytes()) {
            eprintln!("Error writing to file: {}", e);
            return;
        }
    }
    // Note: file.flush() is automatically called when 'file' goes out of scope,
    // but we can call it explicitly if needed:
    // let _ = file.flush();
}

/// Loads a vector of Book structs from a specified file.
/// Assumes each line is formatted as "Title,Author,Year".
fn load_books(filename: &str) -> Vec<Book> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file {}: {}", filename, e);
            return Vec::new(); // Return empty vector on failure
        }
    };

    let reader = BufReader::new(file);
    let mut books = Vec::new();

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Error reading line from file: {}", e);
                continue; // Skip to the next line
            }
        };

        // Split the line by the comma delimiter
        let parts: Vec<&str> = line.splitn(3, ',').collect();

        // Ensure we have exactly three parts
        if parts.len() == 3 {
            let title = parts[0].trim().to_string();
            let author = parts[1].trim().to_string();
            
            // Parse the year string to a u16 integer
            match parts[2].trim().parse::<u16>() {
                Ok(year) => {
                    books.push(Book { title, author, year });
                },
                Err(e) => {
                    eprintln!("Error parsing year '{}': {}", parts[2], e);
                    // Continue to the next book if parsing fails
                }
            }
        } else {
            eprintln!("Skipping badly formatted line: {}", line);
        }
    }

    books
}

fn main() {
    // Create initial book data
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
        Book { title: "The Martian".to_string(), author: "Andy Weir".to_string(), year: 2011 },
    ];

    // --- SAVE OPERATION ---
    let filename = "books.txt";
    save_books(&books, filename);
    println!("Books successfully saved to {}.", filename);

    // --- LOAD OPERATION ---
    let loaded_books = load_books(filename);
    
    // Check if any books were loaded
    if loaded_books.is_empty() {
        println!("No books were loaded from the file or an error occurred.");
    } else {
        println!("\nLoaded books from {}:", filename);
        // Print the loaded book details
        for book in loaded_books {
            println!("- Title: {} | Author: {} | Year: {}", book.title, book.author, book.year);
        }
    }
}
