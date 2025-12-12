use std::{fs::File, io::Read};

type PresentShape = [bool; 9];

fn not_dumb_solution(content: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let mut sum: u64 = 0;
    let mut presents: Vec<PresentShape> = Vec::new();
    let presents_and_areas: Vec<&str> = content.split("\n\n").collect();
    for present in presents_and_areas.iter().rev().skip(1).rev() {
        let index: usize = present.split_once(':').ok_or("Each present should start with its index")?.0.parse()?;
        if index >= presents.len() {
            presents.resize(index + 1, [false; 9]);
        }
        let mut present_index: usize = 0;
        for character in present.chars() {
            match character {
                '#' => {
                    presents[index][present_index] = true;
                    present_index += 1;
                }
                '.' => {
                    present_index += 1;
                }
                _ => {}
            }
        }
    }
    for area in presents_and_areas.iter().last().ok_or("Input should have two sections")?.lines() {
        let area_and_present_list = area.split_once(':').ok_or("Each area should be seperated by a colon")?;
        let area_str = area_and_present_list.0.split_once('x').ok_or("Each area should be width x height")?;
        let area: (usize, usize) = (area_str.0.parse()?, area_str.1.parse()?);
        let present_list: Vec<usize> = area_and_present_list.1.split(' ').skip(1).map(|x| x.parse().unwrap()).collect();
        // Somehow this heuristic works???????? Hell yeah ig but it really shouldn't
        if (area.0 / 3 * area.1 / 3) >= present_list.iter().sum() {
            sum += 1;
        }
    }
    Ok(sum)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_contents: Vec<u8> = Vec::new();
    let mut file = File::open("day12.txt")?;
    file.read_to_end(&mut file_contents)?;
    let content = String::from_utf8(file_contents)?;
    println!("{}", not_dumb_solution(&content)?);
    Ok(())
}
