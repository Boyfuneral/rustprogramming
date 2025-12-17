use std::fs;
use std::io::Write;

fn main() {
    let books_dir = "books";
    let output_file = "output/summary.csv";

    fs::create_dir_all("output").expect("Failed to create output folder");

    let mut rows: Vec<String> = Vec::new();
    rows.push("filename,bytes,lines,words".to_string());

    let mut processed = 0usize;

    let entries = fs::read_dir(books_dir).expect("Could not read 'books' folder");

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("txt") {
            continue;
        }

        let contents = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let bytes = contents.as_bytes().len();
        let lines = contents.lines().count();
        let words = contents.split_whitespace().count();

        let filename = path.file_name().unwrap().to_string_lossy().to_string();
        rows.push(format!("{},{},{},{}", filename, bytes, lines, words));
        processed += 1;
    }

    let mut file = fs::File::create(output_file).expect("Failed to create summary.csv");
    for line in rows {
        writeln!(file, "{}", line).unwrap();
    }

    println!("Processed {} book files.", processed);
    println!("Wrote results to {}", output_file);

    if processed < 100 {
        println!("WARNING: Need at least 100 books. Currently: {}", processed);
    }
}
