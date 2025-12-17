mod thread_pool;
mod analysis;
mod progress;

use analysis::FileAnalysis;
use progress::ProgressEvent;
use thread_pool::ThreadPool;

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc, Mutex,
};

fn main() {
    fs::create_dir_all("output").ok();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let dirs = if args.is_empty() {
        vec!["books".to_string()]
    } else {
        args
    };

    let paths: Vec<PathBuf> = dirs.iter().map(PathBuf::from).collect();
    let files = analysis::collect_txt_files_from_dirs(&paths);

    println!("Found {} .txt files", files.len());

    let cancel = Arc::new(AtomicBool::new(false));
    let (tx, rx) = mpsc::channel::<ProgressEvent>();
    let results: Arc<Mutex<Vec<FileAnalysis>>> = Arc::new(Mutex::new(Vec::new()));

    let mut pool = ThreadPool::new(8);

    for path in files {
        let tx = tx.clone();
        let cancel = cancel.clone();
        let results = results.clone();

        pool.submit(move || {
            let name = path.display().to_string();

            if cancel.load(Ordering::Relaxed) {
                let _ = tx.send(ProgressEvent::Cancelled(name));
                return;
            }

            let _ = tx.send(ProgressEvent::Started(name.clone()));
            let analysis = analysis::analyze_file(&path);
            results.lock().unwrap().push(analysis);
            let _ = tx.send(ProgressEvent::Finished(
                name,
                std::time::Duration::from_millis(0),
            ));
        });
    }

    drop(tx);

    while let Ok(event) = rx.recv() {
        match event {
            ProgressEvent::Started(f) => println!("START {}", f),
            ProgressEvent::Finished(f, _) => println!("DONE {}", f),
            ProgressEvent::Failed(f, m) => eprintln!("FAIL {} {}", f, m),
            ProgressEvent::Cancelled(f) => eprintln!("CANCEL {}", f),
        }
    }

    pool.shutdown();

    let analyses = results.lock().unwrap();
    let mut out = fs::File::create("output/summary.csv").unwrap();
    writeln!(out, "filename,lines,words,bytes").unwrap();

    for a in analyses.iter() {
        if let Some(s) = &a.stats {
            writeln!(
                out,
                "{},{},{},{}",
                a.filename, s.line_count, s.word_count, s.size_bytes
            )
            .unwrap();
        }
    }

    println!("DONE.");
}
