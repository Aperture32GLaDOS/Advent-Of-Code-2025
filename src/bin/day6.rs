use std::{fs::File, io::Read};

fn not_dumb_solution(content: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let mut sum = 0;
    let mut operands: Vec<Vec<u64>> = Vec::new();
    // Find spacings between the operators to obtain the sizes of the columns
    let operators_line = content.lines().last().ok_or("Unable to get last line of content")?;
    let mut spacings: Vec<usize> = operators_line.split(['*', '+']).map(|x| x.len()).filter(|x| *x > 0).collect();
    // Since there is no operator after the terminal operator, we correct for an off-by-one error
    *spacings.last_mut().unwrap() += 1;
    for line in content.lines() {
        let mut index = 0;
        let mut counter = 0;
        while index < line.len() {
            let operand_or_operator = line.split_at(index).1.split_at(spacings[counter]).0;
            let operand = operand_or_operator.trim().parse::<u64>();
            match operand {
                Ok(x) => {
                    if counter == operands.len() {
                        operands.push(Vec::with_capacity(spacings[counter]));
                    }
                    // Initialize the operands to all being 0
                    if operands[counter].len() < spacings[counter] {
                        operands[counter].resize(spacings[counter], 0);
                    }
                    // We skip n columns (where n is the number of whitespace characters at the end
                    // of the operand)
                    let mut column_index = operand_or_operator.len() - operand_or_operator.trim_end().len();
                    let mut new_column_value = x;
                    while new_column_value != 0 {
                        operands[counter][column_index] *= 10;
                        operands[counter][column_index] += new_column_value % 10;
                        new_column_value /= 10;
                        column_index += 1;
                    }
                }
                Err(_) => {
                    let operator = operand_or_operator.trim();
                    let mut local_sum: u64 = 1;
                    match operator {
                        "+" => {
                            local_sum = 0;
                            for operand in operands[counter].iter() {
                                local_sum += operand;
                            }
                        }
                        "*" => {
                            for operand in operands[counter].iter() {
                                local_sum *= operand;
                            }
                        }
                        _ => {
                            return Err("Unknown error".into());
                        }
                    }
                    sum += local_sum;
                }
            }
            index += spacings[counter] + 1;
            counter += 1;
        }
    }
    Ok(sum)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_contents: Vec<u8> = Vec::new();
    let mut file = File::open("day6.txt")?;
    file.read_to_end(&mut file_contents)?;
    let content = String::from_utf8(file_contents)?;
    println!("{}", not_dumb_solution(&content)?);
    Ok(())
}
