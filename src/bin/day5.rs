use std::{fs::File, io::Read, iter::repeat};

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
struct Range {
    low: u64,
    high: u64
}

impl Range {
    fn new(low: u64, high: u64) -> Self {
        Range { low, high }
    }

    fn overlap_with_other(&self, other: &Range) -> Option<Range> {
        if self.is_number_in_range(other.low) {
            if other.high >= self.high {
                return Some(Range::new(self.low, other.high));
            }
            else {
                return Some(*self);
            }
        }
        if self.is_number_in_range(other.high) {
            if other.low <= self.low {
                return Some(Range::new(other.low, self.high));
            }
            else {
                return Some(*self);
            }
        }
        if other.is_number_in_range(self.low) && other.is_number_in_range(self.high) {
            return Some(*other);
        }
        None
    }

    // Inclusive range
    fn is_number_in_range(&self, value: u64) -> bool {
        return self.low <= value && value <= self.high
    }

    fn how_many_in_range(&self) -> u64 {
        self.high - self.low + 1
    }

    fn fold_in(&self, ranges: &mut Vec<Range>) {
        let mut to_add = *self;
        let mut to_remove: Option<usize>;
        let mut change_made = true;
        while change_made {
            change_made = false;
            to_remove = None;
            for (index, range) in ranges.iter().enumerate() {
                let overlap = range.overlap_with_other(&to_add);
                match overlap {
                    Some(x) => {
                        to_add = x;
                        to_remove = Some(index);
                        change_made = true;
                    }
                    None => {}
                }
            }
            match to_remove {
                Some(x) => {
                    ranges.remove(x);
                }
                None => {}
            }
        }
        ranges.push(to_add);
    }
}

fn not_dumb_solution(content: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let mut ranges: Vec<Range> = Vec::new();
    let (range_lines, _numbers_input) = content.split_once("\n\n").ok_or("Unable to split input string")?;
    for range in range_lines.lines() {
        let (range_low_str, range_high_str) = range.split_once('-').ok_or("Unable to split range into two numbers")?;
        let range = Range::new(range_low_str.parse()?, range_high_str.parse()?);
        range.fold_in(&mut ranges);
    }
    Ok(ranges.iter().map(|x| x.how_many_in_range()).sum())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_contents: Vec<u8> = Vec::new();
    let mut file = File::open("day5.txt")?;
    file.read_to_end(&mut file_contents)?;
    let content = String::from_utf8(file_contents)?;
    println!("{}", not_dumb_solution(&content)?);
    Ok(())
}
