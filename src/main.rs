use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod ai;

const CHUNK_SIZE: usize = 1024 * 3;

fn percent_left(current_chunk: &str, chunk_size: usize) -> usize {
    let remaining_size = chunk_size - current_chunk.len();
    (remaining_size as f64 / chunk_size as f64 * 100.0) as usize
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: cargo run -- <open ai key> <filename>");
        return Ok(())
    }

    // Open the file
    let key = &args[1];

    let file = match File::open(&args[2]) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Failed to open file: {}", error);
            return Ok(())
        }
    };

    // Read the file line by line
    let reader = BufReader::new(file);
    let mut chunks: Vec<String> = Vec::new();
    let mut current_chunk = String::new();
    let mut current_size = 0;

    let mut is_previous_line_empty = false;
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(error) => {
                eprintln!("Failed to read line: {}", error);
                continue;
            }
        };

        let line_length = line.len();
        if current_size + line_length > CHUNK_SIZE {
            // Push the current chunk into the array and start a new one
            chunks.push(current_chunk);
            current_chunk = String::new();
            current_size = 0;
        }

        current_chunk.push_str(&line);
        current_size += line_length;

        let two_blank_lines = line.trim().is_empty() && is_previous_line_empty;

        if percent_left(&current_chunk, CHUNK_SIZE) < 20 && !two_blank_lines {
            // Push the current chunk into the array and start a new one
            chunks.push(current_chunk);
            current_chunk = String::new();
            current_size = 0;
        }

        is_previous_line_empty = line.trim().is_empty();
    }

    // Push the last chunk into the array if it's not empty
    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }

    let results = ai::process_chunks(&key, chunks).await?;

    for (index, result) in results.iter().enumerate() {
        println!("Response {}: {}", index + 1, result.message().content);
    }

    Ok(())
}
