use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod ai;

const CHUNK_SIZE: usize = 20000;
const CHUNK_BATCH_SIZE: usize = 5;

fn percent_left(current_chunk: &str, chunk_size: usize) -> usize {
    let remaining_size = chunk_size - current_chunk.len();
    (remaining_size as f64 / chunk_size as f64 * 100.0) as usize
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: ai_format <open ai key> <filename> <prompt>");
        return Ok(());
    }

    let key = &args[1];

    // Open the file
    let file = match File::open(&args[2]) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Failed to open file: {}", error);
            return Ok(());
        }
    };

    let prompt = &args[3];

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

    // split chunks into a nested array of 5 chunk batches
    let mut batched_chunks: Vec<Vec<String>> = chunks
        .chunks(CHUNK_BATCH_SIZE)
        .map(|chunk| chunk.to_vec())
        .collect();

    let mut results: Vec<String> = Vec::new();


    // Send each chunk in batched_chunks to the AI in sequence
    while let Some(chunk) = batched_chunks.first() {
        let result = ai::process_chunks(
            &key,
            prompt.clone(),
            chunk.to_vec(),
        )
        .await?;

        for (_index, result) in result.iter().enumerate() {
            results.push(result.message().content.clone());
        }

        batched_chunks.remove(0);
    }

    println!("{}", results.join("\n"));

    Ok(())
}
