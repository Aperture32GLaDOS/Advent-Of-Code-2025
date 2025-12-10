use std::{collections::HashSet, fs::File, io::Read, mem::swap};

type WiringSchematic = Vec<usize>;

#[derive(Debug)]
#[derive(Clone)]
struct LightProblem {
    lights: Vec<bool>,
    desired_state: Vec<bool>,
    schematics: Vec<WiringSchematic>,
    schematics_used: Vec<usize>
}

impl LightProblem {
    fn new(line: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut lights: Vec<bool> = Vec::new();
        let mut schematics: Vec<WiringSchematic> = Vec::new();
        for character in line.split_once(' ').ok_or("Line must have values separated by spaces")?.0.chars() {
            match character {
                '[' => {continue;}
                ']' => {continue;}
                '.' => {
                    lights.push(false);
                }
                '#' => {
                    lights.push(true);
                }
                _ => {
                    return Err("Unknown character encountered!".into());
                }
            }
            for schematic in line.split('(').skip(1) {
                let schematic_values = schematic.split_once(')').ok_or("Wiring schematics must be surrounded by brackets")?.0;
                let mut new_schematic: WiringSchematic = Vec::new();
                for index in schematic_values.split(',') {
                    new_schematic.push(index.parse()?);
                }
                schematics.push(new_schematic);
            }
        }
        let mut initial_lights = Vec::with_capacity(lights.len());
        initial_lights.resize(lights.len(), false);
        Ok(LightProblem { desired_state: lights, schematics, schematics_used: Vec::new(), lights: initial_lights})
    }

    // Works, but is *far* too slow
    fn naive_breadth_first_solve(&self) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
        let mut subproblems: Vec<Self> = Vec::new();
        subproblems.push(self.clone());
        let mut new_subproblems: Vec<Self> = Vec::with_capacity(subproblems.len());
        // While some lights are still off...
        while subproblems.iter().all(|x| x.lights.iter().zip(x.desired_state.iter()).all(|x| x.0 == x.1)) {
            for subproblem in subproblems.iter() {
                for (schematic_index, possible_schematic) in self.schematics.iter().enumerate() {
                    let mut new_subproblem = subproblem.clone();
                    for light_to_change in possible_schematic.iter() {
                        new_subproblem.lights[*light_to_change] = !new_subproblem.lights[*light_to_change];
                    }
                    new_subproblem.schematics_used.push(schematic_index);
                    new_subproblems.push(new_subproblem);
                }
            }
            swap(&mut subproblems, &mut new_subproblems);
            new_subproblems.clear();
        }
        let solved_subproblem = subproblems.iter().find(|x| x.lights.iter().all(|x| *x)).ok_or("No subproblem found")?;
        Ok(solved_subproblem.schematics_used.clone())
    }

    fn naive_breadth_first_solve_with_state_remembering(&self) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
        let mut subproblems: Vec<Self> = Vec::new();
        subproblems.push(self.clone());
        let mut new_subproblems: Vec<Self> = Vec::with_capacity(subproblems.len());
        let mut found_states: HashSet<Vec<bool>> = HashSet::new();
        let mut depth: usize = 1;
        // While some lights are still off...
        while subproblems.iter().all(|x| !x.lights.iter().zip(x.desired_state.iter()).all(|x| x.0 == x.1)) {
            for subproblem in subproblems.iter() {
                for (schematic_index, possible_schematic) in self.schematics.iter().enumerate() {
                    let mut new_subproblem = subproblem.clone();
                    for light_to_change in possible_schematic.iter() {
                        new_subproblem.lights[*light_to_change] = !new_subproblem.lights[*light_to_change];
                    }
                    match found_states.get(&new_subproblem.lights) {
                        Some(_) => {
                            continue;
                        }
                        None => {
                            new_subproblem.schematics_used.push(schematic_index);
                            found_states.insert(new_subproblem.clone().lights);
                            new_subproblems.push(new_subproblem);
                        }
                    }
                }
            }
            swap(&mut subproblems, &mut new_subproblems);
            new_subproblems.clear();
            depth += 1;
        }
        let solved_subproblem = subproblems.iter().find(|x| x.lights.iter().zip(x.desired_state.iter()).all(|x| x.0 == x.1)).ok_or("No subproblem found")?;
        Ok(solved_subproblem.schematics_used.clone())
    }
}

fn not_dumb_solution(content: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let mut sum = 0;
    for line in content.lines() {
        let problem = LightProblem::new(line)?;
        let solution = problem.naive_breadth_first_solve_with_state_remembering()?;
        sum += solution.len() as u64;
    }
    Ok(sum)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file_contents: Vec<u8> = Vec::new();
    let mut file = File::open("day10.txt")?;
    file.read_to_end(&mut file_contents)?;
    let content = String::from_utf8(file_contents)?;
    println!("{}", not_dumb_solution(&content)?);
    Ok(())
}
