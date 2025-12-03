use std::{fs::File, io::Read};

// Const parameter so loop unrolling can be done
fn get_best_combination<const NUM_DIGITS: usize>(digits_with_indices: &Vec<(usize, u8)>, mut current_index: usize, current_combination: &mut Vec<u8>) {
    for i in (1..NUM_DIGITS+1).rev() {
        let largest_after_current = *digits_with_indices.split_at(digits_with_indices.len() - i + 1).0.split_at(current_index).1.iter().rev().max_by_key(|x| x.1).unwrap();
        if i == 1 {
            current_combination.push(largest_after_current.1);
            return;
        }
        // If the largest digit after the current one can fit the number of remaining digits,
        if largest_after_current.0 <= digits_with_indices.len() - i {
            // Then it is optimal - push it and solve the sub-problem
            current_combination.push(largest_after_current.1);
            current_index = largest_after_current.0 + 1;
            continue;
        }
        // Traverse the array going backwards - keep searching for the max element behind the pointer,
        // until that max element can fit the required number of digits
        let mut pointer = largest_after_current.0;
        let mut current_element = digits_with_indices[pointer];
        while pointer > current_index && current_element.0 > digits_with_indices.len() - i {
            current_element = *digits_with_indices.split_at(pointer).0.split_at(current_index).1.iter().rev().max_by_key(|x| x.1).unwrap_or(&(0, 0));
            pointer = current_element.0;
        }
        current_combination.push(current_element.1);
        current_index = current_element.0 + 1;
    }
}


// Calculates four digits of keys, for testing purposes (very slow)
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
                for digit_three in digits.iter().enumerate() {
                    if digit_two.0 >= digit_three.0 {
                        continue;
                    }
                    for digit_four in digits.iter().enumerate() {
                        if digit_three.0 >= digit_four.0 {
                            continue;
                        }
                        for digit_five in digits.iter().enumerate() {
                            if digit_four.0 >= digit_five.0 {
                                continue;
                            }
                            let combination = (*digit_one.1 as u64) * 10000 + (*digit_two.1 as u64) * 1000 + (*digit_three.1 as u64) * 100 + (*digit_four.1 as u64) * 10 + (*digit_five.1 as u64);
                            if combination > largest_combination {
                                largest_combination = combination;
                            }
                        }
                    }
                }
            }
        }
        sum += largest_combination;
    }
    Ok(sum)
}

fn not_dumb_solution<const NUM_DIGITS: usize>(input: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let mut sum: u64 = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let mut digits: Vec<u8> = Vec::with_capacity(100);
        for digit in line.trim().chars() {
            digits.push(digit.to_digit(10).unwrap() as u8);
        }
        let digits_with_indices: Vec<(usize, u8)> = digits.iter().enumerate().map(|x| (x.0, *x.1)).collect();
        let mut best_combination = Vec::with_capacity(NUM_DIGITS);
        get_best_combination::<NUM_DIGITS>(&digits_with_indices, 0, &mut best_combination);
        let mut multiplier = 1;
        best_combination.reverse();
        for i in best_combination {
            sum += (i as u64) * multiplier;
            multiplier *= 10;
        }
    }
    Ok(sum)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_contents: Vec<u8> = Vec::new();
    let mut file = File::open("day3.txt")?;
    file.read_to_end(&mut file_contents)?;
    let content = String::from_utf8(file_contents)?;
    let now = std::time::Instant::now();
    println!("{}", not_dumb_solution::<12>(&content)?);
    println!("{:?}", now.elapsed());
    Ok(())
}
