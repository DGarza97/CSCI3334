use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::process::Command;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::{Duration, Instant};

struct FileStats {
    word_count: usize,
    line_count: usize,
    char_frequencies: HashMap<char, usize>,
    size_bytes: u64,
}

struct FileAnalysis {
    filename: String,
    stats: FileStats,
    errors: Vec<ProcessingError>,
    processing_time: Duration,
}

enum ProcessingError {
    FileReadError { file: String, msg: String },
    AnalysisError { file: String, msg: String },
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

struct Worker {
    thread: Option<thread::JoinHandle<()>>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for _ in 0..size {
            let r = Arc::clone(&receiver);
            let thread = thread::spawn(move || {
                while let Ok(job) = r.lock().unwrap().recv() {
                    job();
                }
            });
            workers.push(Worker { thread: Some(thread) });
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        if let Some(sender) = &self.sender {
            let _ = sender.send(Box::new(f));
        }
    }

    fn shutdown(&mut self) {
        self.sender.take(); 
        for worker in &mut self.workers {
            if let Some(t) = worker.thread.take() {
                let _ = t.join();
            }
        }
    }
}

fn download_book(url: &str, filename: &str) -> std::io::Result<()> {
    let status = Command::new("curl")
        .arg("-s")
        .arg(url)
        .arg("-o")
        .arg(filename)
        .status()?;

    if !status.success() {
        eprintln!("Failed to download {} from {}", filename, url);
    }

    Ok(())
}

fn download_books(ids: &[u32]) -> std::io::Result<()> {
    fs::create_dir_all("books")?;

    for id in ids {
        let url = format!("https://www.gutenberg.org/files/{0}/{0}-0.txt", id);
        let filename = format!("books/{}.txt", id);
        println!("Downloading {}...", filename);
        let _ = download_book(&url, &filename);
    }

    Ok(())
}

fn analyze_file(filename: &str, cancel: &Arc<Mutex<bool>>) -> FileAnalysis {
    let start = Instant::now();
    let mut errors = Vec::new();
    let mut word_count = 0;
    let mut line_count = 0;
    let mut char_frequencies = HashMap::new();

    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => {
            return FileAnalysis {
                filename: filename.to_string(),
                stats: FileStats {
                    word_count: 0,
                    line_count: 0,
                    char_frequencies: HashMap::new(),
                    size_bytes: 0,
                },
                errors: vec![ProcessingError::FileReadError {
                    file: filename.to_string(),
                    msg: e.to_string(),
                }],
                processing_time: start.elapsed(),
            };
        }
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        if *cancel.lock().unwrap() {
            break;
        }
        match line {
            Ok(l) => {
                line_count += 1;
                word_count += l.split_whitespace().count();
                for c in l.chars() {
                    *char_frequencies.entry(c).or_insert(0) += 1;
                }
            }
            Err(e) => errors.push(ProcessingError::AnalysisError {
                file: filename.to_string(),
                msg: e.to_string(),
            }),
        }
    }

    let size_bytes = fs::metadata(filename).map(|m| m.len()).unwrap_or(0);

    FileAnalysis {
        filename: filename.to_string(),
        stats: FileStats {
            word_count,
            line_count,
            char_frequencies,
            size_bytes,
        },
        errors,
        processing_time: start.elapsed(),
    }
}

fn top_chars(freqs: &HashMap<char, usize>, n: usize) -> Vec<(char, usize)> {
    let mut items: Vec<(char, usize)> = freqs.iter().map(|(c, v)| (*c, *v)).collect();
    items.sort_by(|a, b| b.1.cmp(&a.1));
    items.truncate(n);
    items
}

fn collect_files(dir: &str) -> Vec<String> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("txt") {
                if let Some(p) = path.to_str() {
                    files.push(p.to_string());
                }
            }
        }
    }
    files
}

fn main() -> std::io::Result<()> {
    let book_ids: Vec<u32> = vec![
        84, 2701, 1342, 1513, 43, 8492, 46, 11, 100, 145,
        2641, 37106, 2554, 25344, 1260, 67979, 16389, 16328, 
        345, 174, 394, 2542, 2160, 6761, 6593, 4085, 1259, 844,
        5197, 64317, 1080, 98, 76, 1661, 768, 1184, 28054, 3207, 
        1400, 1998, 205, 1232, 2591, 4300, 5200, 74, 33944, 36034,
        55, 6130, 2600, 3296, 16119, 7370, 45, 4363, 8800, 25162, 
        1023, 17135, 5740, 3206, 17199, 120, 1952, 408, 2852, 
        41445, 1727, 1399, 72679, 34901, 66957, 23, 2148, 829,
        1497, 514, 41, 26, 161, 135, 779, 30254, 730, 27827, 
        2680, 16, 996, 244, 15399, 219, 56517, 110, 42324, 19942,
        10554, 4280, 10676, 8438
    ];

    download_books(&book_ids)?;

    let cancel_flag = Arc::new(Mutex::new(false));
    let progress = Arc::new(Mutex::new(0usize));

    {
        let cancel = Arc::clone(&cancel_flag);
        thread::spawn(move || {
            println!("Press ENTER to cancel processing...");
            let mut input = String::new();
            let _ = io::stdin().read_line(&mut input);
            *cancel.lock().unwrap() = true;
        });
    }

    let files = collect_files("books");
    let total = files.len();
    println!("Discovered {} text files", total);
    let pool = ThreadPool::new(8);
    let results = Arc::new(Mutex::new(Vec::<FileAnalysis>::new()));

    for file in files {
        let results = Arc::clone(&results);
        let cancel = Arc::clone(&cancel_flag);
        let progress = Arc::clone(&progress);

        pool.execute(move || {
            let analysis = analyze_file(&file, &cancel);
            results.lock().unwrap().push(analysis);

            let mut p = progress.lock().unwrap();
            *p += 1;
            println!("Progress: {}/{}", *p, total);
        });
    }

    let mut pool = pool;
    pool.shutdown();

    let results = results.lock().unwrap();
    println!("\n--- Final Analysis Results ---");
    for r in results.iter() {
        println!("File: {}", r.filename);
        println!("  Lines: {}", r.stats.line_count);
        println!("  Words: {}", r.stats.word_count);
        println!("  Size: {} bytes", r.stats.size_bytes);
        println!("  Time: {:.2?}", r.processing_time);

        let top = top_chars(&r.stats.char_frequencies, 5);
        print!("  Top characters: ");
        for (c, count) in top {
            print!("'{}': {}  ", c, count);
        }
        println!();

        for err in &r.errors {
            match err {
                ProcessingError::FileReadError { file, msg } => {
                    println!("  Read error in {}: {}", file, msg);
                }
                ProcessingError::AnalysisError { file, msg } => {
                    println!("  Analysis error in {}: {}", file, msg);
                }
            }
        }
        println!("-----------------------------");
    }

    println!("Processing complete.");
    Ok(())
}
