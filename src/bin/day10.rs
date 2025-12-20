use std::{fs::File, io::Read};
use good_lp::{constraint, microlp, variable, Expression, ProblemVariables, SolverModel, Variable};

type WiringSchematic = Vec<u64>;

#[derive(Debug)]
#[derive(Clone)]
struct LightProblem {
    desired_state: Vec<u64>,
    schematics: Vec<WiringSchematic>
}

impl LightProblem {
    fn new(line: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut desired_state: Vec<u64> = Vec::new();
        let mut schematics: Vec<WiringSchematic> = Vec::new();
        for counter_state in line.split_once('{').ok_or("Desired counter state must be contained in curly braces")?.1.split_once('}').ok_or("Desired counter state must be contained in curly braces")?.0.split(',') {
            desired_state.push(counter_state.parse()?);
        }
        for schematic in line.split('(').skip(1) {
            let schematic_values = schematic.split_once(')').ok_or("Wiring schematics must be surrounded by brackets")?.0;
            let mut new_schematic: WiringSchematic = Vec::with_capacity(desired_state.len());
            new_schematic.resize(desired_state.len(), 0);
            for index in schematic_values.split(',') {
                new_schematic[index.parse::<usize>()?] = 1;
            }
            schematics.push(new_schematic);
        }
        Ok(LightProblem { desired_state, schematics })
    }

    fn solve(&self) -> Result<u64, Box<dyn std::error::Error>> {
        // If only I could solve this without linear programming solvers... :(
        let mut problem = ProblemVariables::new();
        let mut coefficients: Vec<Variable> = Vec::with_capacity(self.schematics.len());
        for _ in 0..self.schematics.len() {
            coefficients.push(problem.add(variable().integer().min(0)));
        }
        let sum = coefficients.iter().sum::<Expression>();
        let mut counter_states: Vec<Expression> = Vec::with_capacity(self.desired_state.len());
        for i in 0..self.desired_state.len() {
            let mut counter_state: Expression = Expression::from(0);
            for (schematic, schematic_coefficient) in self.schematics.iter().zip(coefficients.iter()) {
                counter_state += schematic[i] as f64 * *schematic_coefficient;
            }
            counter_states.push(counter_state);
        }
        let mut solver = problem.minimise(&sum).using(microlp);
        for (state, desired) in counter_states.iter().zip(self.desired_state.iter()) {
            solver = solver.with(constraint!(state.clone() == *desired as f64));
        }
        let solution = solver.solve()?;
        Ok(sum.eval_with(&solution) as u64)
    }
}

fn not_dumb_solution(content: &String) -> Result<u64, Box<dyn std::error::Error>> {
    let mut sum = 0;
    for line in content.lines() {
        let problem = LightProblem::new(line)?;
        let solution = problem.solve()?;
        sum += solution;
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
