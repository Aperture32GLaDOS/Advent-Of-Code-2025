use std::{fs::File, io::Read, mem::swap};

struct Circuit {
    parent: usize,
    size: usize
}

// Merges two circuits, returning the size of the newly merged circuit
fn merge(circuits: &mut [Circuit], mut first_junctionbox: usize, mut second_junctionbox: usize) -> usize {
    // Find a pair of circuits, both of which have not been merged
    while circuits[first_junctionbox].parent != first_junctionbox {
        let parent = circuits[first_junctionbox].parent;
        (first_junctionbox, circuits[first_junctionbox].parent) = (parent, circuits[parent].parent);
    }
    while circuits[second_junctionbox].parent != second_junctionbox {
        let parent = circuits[second_junctionbox].parent;
        (second_junctionbox, circuits[second_junctionbox].parent) = (parent, circuits[parent].parent);
    }
    if first_junctionbox != second_junctionbox {
        // Always make the smaller circuit the parent
        if circuits[first_junctionbox].size < circuits[second_junctionbox].size {
            swap(&mut first_junctionbox, &mut second_junctionbox);
        }
        circuits[second_junctionbox].parent = first_junctionbox;
        circuits[first_junctionbox].size += circuits[second_junctionbox].size;
    }
    circuits[first_junctionbox].size
}

fn get_dist_between_points(point_one: (u64, u64, u64), point_two: (u64, u64, u64)) -> u64 {
    (point_one.0.abs_diff(point_two.0)).pow(2) + (point_one.1.abs_diff(point_two.1)).pow(2) + (point_one.2.abs_diff(point_two.2)).pow(2)
}

fn get_pairs_brute_force(points: &Vec<(u64, u64, u64)>) -> Result<Vec<(usize, usize, u64)>, Box<dyn std::error::Error>> {
    let mut pairs_with_dist = Vec::new();
    for (index_one, point_one) in points.iter().enumerate() {
        // Skip the first index_one + 1 (to discount duplicate pairs)
        for (index_two, point_two) in points.iter().enumerate().skip(index_one + 1) {
            if point_two == point_one {
                continue;
            }
            let distance = get_dist_between_points(*point_one, *point_two);
            pairs_with_dist.push((index_one, index_two, distance));
        }
    }
    Ok(pairs_with_dist)
}

fn not_dumb_solution(content: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let mut positions: Vec<(u64, u64, u64)> = Vec::new();
    for line in content.lines() {
        let mut position_string = line.split(',');
        let position = (position_string.next().ok_or("Each line should contain 3 numbers")?.parse::<u64>()?,
            position_string.next().ok_or("Each line should contain 3 numbers")?.parse::<u64>()?,
            position_string.next().ok_or("Each line should contain 3 numbers")?.parse::<u64>()?);
        positions.push(position);
    }
    let mut pairs = get_pairs_brute_force(&positions)?;
    pairs.sort_by_key(|x| x.2);
    let mut circuits: Vec<Circuit> = (0..pairs.len()).map(|x| Circuit { parent: x, size: 1 }).collect();
    for pair in pairs.iter() {
        let merged_length = merge(&mut circuits, pair.0, pair.1);
        if merged_length == positions.len() {
            return Ok(positions[pair.0].0 * positions[pair.1].0);
        }
    }
    Err("All pairs have been merged, yet more is to merge...?".into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_contents: Vec<u8> = Vec::new();
    let mut file = File::open("day8.txt")?;
    file.read_to_end(&mut file_contents)?;
    let content = String::from_utf8(file_contents)?;
    println!("{}", not_dumb_solution(&content)?);
    Ok(())
}
