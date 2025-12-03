use std::{fs::File, io::Read};

// Recursive method, finds the best digit after and including current_index and then finds the next
// sub-solution of the digits after that best digit
fn get_best_combination(digits_with_indices: &mut Vec<(usize, u8)>, num_digits: usize, current_index: usize, largest_after: Vec<(usize, u8)>, current_combination: &mut Vec<u8>) {
    if num_digits == 1 {
        current_combination.push(largest_after[current_index].1);
        return;
    }
    // Zero out all digits before the current index, so they do not affect largest_before
    digits_with_indices.iter_mut().for_each(|x| {
        if x.0 >= current_index {
            return;
        }
        else {
            *x = (x.0, 0);
        }
    });
    // Vector of the largest element before the given index, preferring elements closer to the
    // start - needs to be re-calculated as it may include digits before current_index
    // Note that largest_after does not need to be re-calculated, as it will only include elements
    // after and including its index, by definition
    let largest_before: Vec<(usize, u8)> = digits_with_indices.iter().map(|x| *digits_with_indices.split_at(x.0).0.iter().rev().max_by_key(|x| x.1).unwrap_or(&(0, 0))).collect();
    let largest_after_current = largest_after[current_index];
    // If the largest digit after the current one can fit the number of remaining digits,
    if largest_after_current.0 <= digits_with_indices.len() - num_digits {
        // Then it is optimal - push it and solve the sub-problem
        current_combination.push(largest_after_current.1);
        get_best_combination(digits_with_indices, num_digits - 1, largest_after_current.0 + 1, largest_after, current_combination);
        return;
    }
    // Otherwise, we traverse the array going backwards from the largest after the current index
    // until the largest index before our pointer will fit the number of digits
    let mut pointer = largest_after_current.0;
    let mut largest_before_largest_after = largest_before[pointer];
    while pointer > current_index && largest_before_largest_after.0 > digits_with_indices.len() - num_digits {
        pointer -= 1;
        largest_before_largest_after = largest_before[pointer];
    }
    current_combination.push(largest_before_largest_after.1);
    get_best_combination(digits_with_indices, num_digits - 1, largest_before_largest_after.0 + 1, largest_after, current_combination);
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

fn not_dumb_solution(input: &String, num_digits: usize) -> Result<u64, Box<dyn std::error::Error>> {
    let mut sum: u64 = 0;
    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let mut digits: Vec<u8> = Vec::new();
        for digit in line.trim().chars() {
            digits.push(digit.to_digit(10).unwrap() as u8);
        }
        let mut digits_with_indices: Vec<(usize, u8)> = digits.iter().enumerate().map(|x| (x.0, *x.1)).collect();
        // Vector of the largest digit after (and including) this index, preferring digits closer
        // to the start
        let largest_after: Vec<(usize, u8)> = digits.iter().enumerate().map(|x| *digits_with_indices.split_at(x.0).1.iter().rev().max_by_key(|x| x.1).unwrap()).collect();
        let mut best_combination = Vec::new();
        get_best_combination(&mut digits_with_indices, num_digits, 0, largest_after, &mut best_combination);
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
    println!("{}", not_dumb_solution(&content, 12)?);
    Ok(())
}
