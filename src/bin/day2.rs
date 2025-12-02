use std::{fs::File, io::Read};

fn check_chunks(input: &String, num_chunks: usize) -> bool {
    let mut chunks: Vec<&str> = Vec::new();
    if input.len() % num_chunks != 0 {
        return false;
    }
    let chunk_size = input.len() / num_chunks;
    for i in 0..num_chunks {
        let split_result = input.split_at(chunk_size * (i + 1)).0;
        let split = split_result.split_at(chunk_size * i).1;
        chunks.push(split);
    }
    let first_chunk = chunks.first().unwrap();
    chunks.iter().all(|x| x == first_chunk)
}

fn dumb_solution(input: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let mut invalid_count: u64 = 0;
    for range in input.trim().split(',') {
        let (range_min_str, range_max_str) = range.split_once('-').unwrap();
        let range_min: u64 = range_min_str.parse()?;
        let range_max: u64 = range_max_str.parse()?;
        // Inclusive range, so add 1
        for i in range_min..range_max+1 {
            let current_number = i.to_string();
            for j in 2..current_number.len()+1 {
                // Check if the number can be split into j chunks of the same number, repeated
                if check_chunks(&current_number, j) {
                    // And if they can, then add the number
                    invalid_count += i;
                    // And break
                    break;
                }
            }
        }
    }
    Ok(invalid_count)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_contents: Vec<u8> = Vec::new();
    let mut file = File::open("day2.txt")?;
    file.read_to_end(&mut file_contents)?;
    let content = String::from_utf8(file_contents)?;
    println!("{}", dumb_solution(&content)?);
    Ok(())
}
