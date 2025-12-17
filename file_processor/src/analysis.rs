use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct ProcessingError {
    pub filename: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct FileStats {
    pub word_count: usize,
    pub line_count: usize,
    pub char_frequencies: HashMap<char, usize>,
    pub size_bytes: u64,
}

#[derive(Debug, Clone)]
pub struct FileAnalysis {
    pub filename: String,
    pub stats: Option<FileStats>,
    pub errors: Vec<ProcessingError>,
    pub processing_time: Duration,
}

pub fn analyze_file(path: &Path) -> FileAnalysis {
    let start = Instant::now();
    let filename = path.display().to_string();
    let mut errors: Vec<ProcessingError> = Vec::new();

    let size_bytes = match std::fs::metadata(path) {
        Ok(m) => m.len(),
        Err(e) => {
            errors.push(ProcessingError {
                filename: filename.clone(),
                message: format!("metadata error: {}", e),
            });
            return FileAnalysis {
                filename,
                stats: None,
                errors,
                processing_time: start.elapsed(),
            };
        }
    };

    let text = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            errors.push(ProcessingError {
                filename: filename.clone(),
                message: format!("read_to_string error: {}", e),
            });
            return FileAnalysis {
                filename,
                stats: None,
                errors,
                processing_time: start.elapsed(),
            };
        }
    };

    let line_count = text.lines().count();
    let word_count = text.split_whitespace().count();

    let mut char_frequencies: HashMap<char, usize> = HashMap::new();
    for ch in text.chars() {
        *char_frequencies.entry(ch).or_insert(0) += 1;
    }

    let stats = FileStats {
        word_count,
        line_count,
        char_frequencies,
        size_bytes,
    };

    FileAnalysis {
        filename,
        stats: Some(stats),
        errors,
        processing_time: start.elapsed(),
    }
}

pub fn collect_txt_files_from_dirs(dirs: &[PathBuf]) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for d in dirs {
        match std::fs::read_dir(d) {
            Ok(entries) => {
                for e in entries.flatten() {
                    let p = e.path();
                    if p.is_file()
                        && p.extension().and_then(|s| s.to_str()) == Some("txt")
                    {
                        out.push(p);
                    }
                }
            }
            Err(_) => {
                // handled by caller; keep going
            }
        }
    }
    out
}
