use std::{fs::File, io::Read, mem::swap};

// Good enough, solves part 2 in around 2.5ms
fn dumb_solution(content: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let mut sum: u64 = 0;
    let size: (usize, usize) = (content.lines().next().ok_or("No line in file")?.len(), content.lines().count());
    let mut state_vector: Vec<u8> = Vec::with_capacity(size.0 * size.1);
    let mut new_state_vector: Vec<u8> = Vec::with_capacity(size.0 * size.1);
    new_state_vector.resize(new_state_vector.capacity(), 0);
    for line in content.lines() {
        for character in line.chars() {
            if character == '@' {
                state_vector.push(1);
                // If toilet roll, check the state vector at locations already added (if they exist)
            }
            else {
                state_vector.push(0);
            }
        }
    }
    let mut change_made = true;
    while change_made {
        change_made = false;
        for i in 0..state_vector.len() {
            new_state_vector[i] = state_vector[i];
            if state_vector[i] == 0 {
                continue;
            }
            let horizontal_index = i % size.0;
            let vertical_index = i / size.0;
            let mut local_sum = 0;
            if vertical_index > 0 {
                local_sum += state_vector[(vertical_index - 1) * size.0 + horizontal_index];
                if horizontal_index > 0 {
                    local_sum += state_vector[(vertical_index - 1) * size.0 + horizontal_index - 1];
                }
                if horizontal_index < size.0 - 1 {
                    local_sum += state_vector[(vertical_index - 1) * size.0 + horizontal_index + 1];
                }
            }
            if vertical_index < size.1 - 1 {
                local_sum += state_vector[(vertical_index + 1) * size.0 + horizontal_index];
                if horizontal_index > 0 {
                    local_sum += state_vector[(vertical_index + 1) * size.0 + horizontal_index - 1];
                }
                if horizontal_index < size.0 - 1 {
                    local_sum += state_vector[(vertical_index + 1) * size.0 + horizontal_index + 1];
                }
            }
            if horizontal_index > 0 {
                local_sum += state_vector[vertical_index * size.0 + horizontal_index - 1];
            }
            if horizontal_index < size.0 - 1 {
                local_sum += state_vector[vertical_index * size.0 + horizontal_index + 1];
            }
            if local_sum < 4 {
                change_made = true;
                new_state_vector[i] = 0;
                sum += 1;
            }
        }
        swap(&mut state_vector, &mut new_state_vector);
    }
    Ok(sum)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_contents: Vec<u8> = Vec::new();
    let mut file = File::open("day4.txt")?;
    file.read_to_end(&mut file_contents)?;
    let content = String::from_utf8(file_contents)?;
    println!("{}", dumb_solution(&content)?);
    Ok(())
}
