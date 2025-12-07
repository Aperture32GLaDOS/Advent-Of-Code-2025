use std::{fs::File, io::Read};

fn not_dumb_solution(content: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let num_spaces = content.lines().next().ok_or("Unable to get first line of content")?.trim().len();
    let mut beams: Vec<bool> = Vec::with_capacity(num_spaces);
    // How many ways are there to reach a point?
    let mut timelines_which_reach_point: Vec<usize> = Vec::with_capacity(num_spaces);
    beams.resize(num_spaces, false);
    timelines_which_reach_point.resize(num_spaces, 0);
    for line in content.lines() {
        for (index, character) in line.chars().enumerate() {
            match character {
                '.' => {}
                'S' => {
                    beams[index] = true;
                    timelines_which_reach_point[index] = 1;
                }
                '^' => {
                    // On split,
                    if beams[index] {
                        beams[index] = false;
                        // Set the number of ways of getting to the locations of the split as the
                        // number of ways of getting to the splitter
                        if index > 0 {
                            beams[index - 1] = true;
                            timelines_which_reach_point[index - 1] += timelines_which_reach_point[index];
                        }
                        if index < num_spaces - 1 {
                            beams[index + 1] = true;
                            timelines_which_reach_point[index + 1] += timelines_which_reach_point[index];
                        }
                        // And the split point is now unreachable
                        timelines_which_reach_point[index] = 0;
                    }
                }
                _ => {
                    return Err(format!("Unknown character encountered: {}", character).into());
                }
            }
        }
    }
    // Sum the ways in which the terminal points can be reached
    Ok(timelines_which_reach_point.iter().sum::<usize>() as u64)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_contents: Vec<u8> = Vec::new();
    let mut file = File::open("day7.txt")?;
    file.read_to_end(&mut file_contents)?;
    let content = String::from_utf8(file_contents)?;
    println!("{}", not_dumb_solution(&content)?);
    Ok(())
}
