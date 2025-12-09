use std::{collections::HashSet, fs::File, io::Read, mem::swap};

type Rectangle = ((u64, u64), (u64, u64));

fn is_rectangle_contained(rectangle: Rectangle, red_tiles: &Vec<(u64, u64)>) -> bool {
    let mut first_x = rectangle.0.0;
    let mut second_x = rectangle.1.0;
    let mut first_y = rectangle.0.1;
    let mut second_y = rectangle.1.1;
    if first_x > second_x {
        swap(&mut first_x, &mut second_x);
    }
    if first_y > second_y {
        swap(&mut first_y, &mut second_y);
    }
    // Check if the lines of the rectangle intersect with any of the boundary lines
    let mut intersects = false;
    for (index, tile) in red_tiles.iter().enumerate() {
        let tile_before: (u64, u64);
        if index == 0 {
            tile_before = red_tiles[red_tiles.len() - 1];
        }
        else {
            tile_before = red_tiles[index - 1];
        }
        // If the boundary line is in the y-direction,
        if tile_before.0 == tile.0 {
            // Check if its x-location is within the rectangle's x-boundaries
            if tile.0 > first_x && tile.0 < second_x {
                // And check if either x-line in the rectangle intersects
                let tile_start_y = tile.1.min(tile_before.1);
                let tile_end_y = tile.1.max(tile_before.1);
                if tile_start_y.max(first_y) < tile_end_y.min(second_y) {
                    intersects = true;
                    break;
                }
            }
        }
        else if tile_before.1 == tile.1 {
            if tile.1 > first_y && tile.1 < second_y {
                let tile_start_x = tile.0.min(tile_before.0);
                let tile_end_x = tile.0.max(tile_before.0);
                if tile_start_x.max(first_x) < tile_end_x.min(second_x) {
                    intersects = true;
                    break;
                }
            }
        }
    }
    if intersects {
        return false;
    }
    true
}

fn not_dumb_solution(content: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let mut red_tiles: Vec<(u64, u64)> = Vec::new();
    for line in content.lines() {
        let coordinates = line.split_once(',').ok_or("Line must be two comma-seperated numbers")?;
        red_tiles.push((coordinates.0.parse()?, coordinates.1.parse()?));
    }
    let mut pairs: Vec<(usize, usize, u64)> = Vec::with_capacity(red_tiles.len() * red_tiles.len() / 2);
    for (index_one, tile_one) in red_tiles.iter().enumerate() {
        for (index_two, tile_two) in red_tiles.iter().enumerate().skip(index_one + 1) {
            let area = (tile_one.0.abs_diff(tile_two.0) + 1) * (tile_one.1.abs_diff(tile_two.1) + 1);
            pairs.push((index_one, index_two, area));
        }
    }
    pairs.sort_unstable_by_key(|x| x.2);
    pairs.reverse();
    for possible_rectangle in pairs.iter() {
        let corner_one = red_tiles[possible_rectangle.0];
        let corner_two = red_tiles[possible_rectangle.1];
        if is_rectangle_contained((corner_one, corner_two), &red_tiles) {
            return Ok(possible_rectangle.2);
        }
    }
    Err("No viable rectangle found".into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_contents: Vec<u8> = Vec::new();
    let mut file = File::open("day9.txt")?;
    file.read_to_end(&mut file_contents)?;
    let content = String::from_utf8(file_contents)?;
    println!("{}", not_dumb_solution(&content)?);
    Ok(())
}
