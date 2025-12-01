// Part 2 solution, as part 1 is trivial
use std::{fs::File, io::Read};

fn dumb_solution(input: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let mut dial_start: i16 = 50;
    let mut num_at_zero: u64 = 0;
    for line in input.lines() {
        if !line.trim().is_empty() {
            let is_negative: bool = line.chars().next().unwrap() == 'L';
            let to_add = line.split_at(1).1.parse::<i16>()?;
            if to_add == 0 {
                continue;
            }
            let mut new_dial = dial_start;
            // Dumb solution just does the addition one-by-one
            for _ in 0..to_add {
                if is_negative {
                    new_dial -= 1;
                }
                else {
                    new_dial += 1;
                }
                if (new_dial % 100 + 100) % 100 == 0 {
                    num_at_zero += 1;
                }
            }
            dial_start = (new_dial % 100 + 100) % 100;
        }
    }
    Ok(num_at_zero)
}

fn not_dumb_solution(input: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let mut dial_start: i16 = 50;
    let mut num_at_zero: u64 = 0;
    for line in input.lines() {
        if !line.trim().is_empty() {
            let is_negative: bool = line.chars().next().unwrap() == 'L';
            let to_add = line.split_at(1).1.parse::<i16>()?;
            if to_add == 0 {
                continue;
            }
            let new_dial = dial_start + (to_add % 100) * if is_negative {-1} else {1};
            // Time for fucky modular arithmetic
            // If we're adding multiples of 100, then the number of zeros will definitely increase
            num_at_zero += (to_add.abs() / 100) as u64;
            // Otherwise, we need to check the result
            if (new_dial <= 0 || new_dial >= 100) && dial_start != 0 {
                num_at_zero += 1;
            }
            dial_start = (new_dial % 100 + 100) % 100;
        }
    }
    Ok(num_at_zero)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_contents: Vec<u8> = Vec::new();
    let mut file = File::open("day1.txt")?;
    file.read_to_end(&mut file_contents)?;
    let content = String::from_utf8(file_contents)?;
    println!("{}", not_dumb_solution(&content)?);
    println!("{}", dumb_solution(&content)?);
    Ok(())
}
