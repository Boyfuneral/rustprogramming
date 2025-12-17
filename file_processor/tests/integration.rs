use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir(name: &str) -> PathBuf {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    std::env::temp_dir().join(format!("{}_{}", name, ts))
}

#[test]
fn file_processor_creates_summary_csv_for_directory() {
    // Create an isolated temp "workspace" so output/summary.csv is written there
    let root = unique_temp_dir("file_processor_it");
    let books = root.join("books");
    let output = root.join("output");
    fs::create_dir_all(&books).unwrap();
    fs::create_dir_all(&output).unwrap();

    // Create a couple test .txt files
    fs::write(books.join("a.txt"), "hello world\nthis is a test\n").unwrap();
    fs::write(books.join("b.txt"), "one two three\n").unwrap();

    // Run the compiled binary (Cargo provides this env var for integration tests)
    let exe = env!("CARGO_BIN_EXE_file_processor");

    let status = Command::new(exe)
        .current_dir(&root)           // so output/ goes into this temp root
        .arg("books")                 // directory name relative to current_dir
        .status()
        .expect("failed to run file_processor binary");

    assert!(status.success(), "program did not exit successfully");

    // Verify output file exists
    let summary = root.join("output").join("summary.csv");
    assert!(summary.exists(), "summary.csv was not created");

    // Verify CSV has header + at least the files we created
    let csv = fs::read_to_string(summary).unwrap();
    assert!(csv.contains("filename,lines,words,bytes"), "missing CSV header");
    assert!(csv.contains("books/a.txt") || csv.contains("a.txt"), "missing a.txt row");
    assert!(csv.contains("books/b.txt") || csv.contains("b.txt"), "missing b.txt row");
}
