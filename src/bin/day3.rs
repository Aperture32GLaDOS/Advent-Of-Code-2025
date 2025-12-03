use std::{fs::File, io::Read, mem::swap};

fn dumb_solution(input: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let mut sum: u64 = 0;
    for line in input.lines() {
        let mut largest_combination: u64 = 0;
        if line.trim().is_empty() {
            continue;
        }
        let mut digits: Vec<u8> = Vec::new();
        for digit in line.trim().chars() {
            digits.push(digit.to_digit(10).unwrap() as u8);
        }
        for digit_one in digits.iter().enumerate() {
            for digit_two in digits.iter().enumerate() {
                if digit_one.0 >= digit_two.0 {
                    continue;
                }
                let combination = (*digit_one.1 as u64) * 10 + (*digit_two.1 as u64);
                if combination > largest_combination {
                    largest_combination = combination;
                }
            }
        }
        sum += largest_combination;
    }
    Ok(sum)
}

fn not_dumb_solution(input: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let mut sum: u64 = 0;
    for line in input.lines() {
        let mut largest_combination: u64 = 0;
        if line.trim().is_empty() {
            continue;
        }
        let mut digits: Vec<u8> = Vec::new();
        for digit in line.trim().chars() {
            digits.push(digit.to_digit(10).unwrap() as u8);
        }
        let digits_with_indices: Vec<(usize, u8)> = digits.iter().enumerate().map(|x| (x.0, *x.1)).collect();
        // Vector of the largest digit after (and including) this index
        let largest_after: Vec<(usize, u8)> = digits.iter().enumerate().map(|x| *digits_with_indices.split_at(x.0).1.iter().rev().max_by_key(|x| x.1).unwrap()).collect();
        // Vector of the largest digit before this index
        let largest_before: Vec<(usize, u8)> = digits.iter().enumerate().map(|x| *digits_with_indices.split_at(x.0).0.iter().rev().max_by_key(|x| x.1).unwrap_or(&(0, 0))).collect();
        for i in 0..(largest_after.len() - 1) {
            if largest_after[i] != largest_after[i + 1] {
                largest_combination = (largest_after[i].1 as u64) * 10 + (largest_after[i + 1].1 as u64);
                break;
            }
        }
        if largest_combination == 0 {
            largest_combination = (largest_before.last().unwrap().1 as u64) * 10 + (largest_after.last().unwrap().1 as u64);
        }
        sum += largest_combination;
    }
    Ok(sum)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_contents: Vec<u8> = Vec::new();
    let mut file = File::open("day3.txt")?;
    file.read_to_end(&mut file_contents)?;
    let content = String::from_utf8(file_contents)?;
    println!("{}", dumb_solution(&content)?);
    println!("{}", not_dumb_solution(&content)?);
    Ok(())
}
