use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::fs;
use std::hash::{Hash, Hasher};
use std::time::Instant;

const PROBLEM_NAME: &str = "Memory Reallocation";
const PROBLEM_INPUT_FILE: &str = "./input/day06.txt";
const PROBLEM_DAY: u64 = 6;

/// Processes the AOC 2017 Day 06 input file and solves both parts of the problem. Solutions are
/// printed to stdout.
pub fn main() {
    let start = Instant::now();
    // Input processing
    let input = process_input_file(PROBLEM_INPUT_FILE);
    let input_parser_timestamp = Instant::now();
    let input_parser_duration = input_parser_timestamp.duration_since(start);
    // Solve part 1
    let p1_solution = solve_part1(&input);
    let p1_timestamp = Instant::now();
    let p1_duration = p1_timestamp.duration_since(input_parser_timestamp);
    // Solve part 2
    let p2_solution = solve_part2(&input);
    let p2_timestamp = Instant::now();
    let p2_duration = p2_timestamp.duration_since(p1_timestamp);
    // Print results
    println!("==================================================");
    println!("AOC 2017 Day {PROBLEM_DAY} - \"{PROBLEM_NAME}\"");
    println!("[+] Part 1: {p1_solution}");
    println!("[+] Part 2: {p2_solution}");
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    println!("Execution times:");
    println!("[+] Input:  {input_parser_duration:.2?}");
    println!("[+] Part 1: {p1_duration:.2?}");
    println!("[+] Part 2: {p2_duration:.2?}");
    println!(
        "[*] TOTAL:  {:.2?}",
        input_parser_duration + p1_duration + p2_duration
    );
    println!("==================================================");
}

/// Processes the AOC 2017 Day 06 input file in the format required by the solver functions.
/// Returned value is vector of values given as whitespace-separated values in the input file.
fn process_input_file(filename: &str) -> Vec<u64> {
    // Read contents of problem input file
    let raw_input = fs::read_to_string(filename).unwrap();
    // Process input file contents into data structure
    raw_input
        .trim()
        .split_ascii_whitespace()
        .map(|value| value.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

/// Solves AOC 2017 Day 06 Part 1 // Determines how many redistribution cycles must be completed
/// before a configuration is produced that has already been observed.
fn solve_part1(banks: &[u64]) -> u64 {
    if banks.is_empty() {
        return 0;
    }
    let mut banks = banks.to_vec();
    let mut observed: HashSet<u64> = HashSet::from([hash_banks(&banks)]);
    let mut steps = 0;
    loop {
        steps += 1;
        conduct_redistribution_cycle(&mut banks);
        // Record banks hash and check if it has already been observed
        if !observed.insert(hash_banks(&banks)) {
            break;
        }
    }
    steps
}

/// Conduct a single redistribution cycle of blocks between the banks.
fn conduct_redistribution_cycle(banks: &mut Vec<u64>) {
    let mut i = find_index_of_largest_bank(banks);
    let mut blocks = banks[i];
    banks[i] = 0;
    while blocks > 0 {
        i = (i + 1) % banks.len();
        banks[i] += 1;
        blocks -= 1;
    }
}

/// Finds the index of the bank with the largest number of blocks. Ties are broken by selecting the
/// bank with the lower-numbered index.
///
/// Input banks must not be empty vector.
fn find_index_of_largest_bank(banks: &[u64]) -> usize {
    let mut i: Option<usize> = None;
    let mut max_value: Option<u64> = None;
    for (j, value) in banks.iter().enumerate() {
        if max_value.is_none() || max_value.unwrap() < *value {
            i = Some(j);
            max_value = Some(*value);
        }
    }
    i.unwrap()
}

/// Solves AOC 2017 Day 06 Part 2 // ###
fn solve_part2(_input: &[u64]) -> u64 {
    unimplemented!();
}

fn hash_banks(banks: &[u64]) -> u64 {
    let mut hasher = DefaultHasher::new();
    banks.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod test {
    use super::*;

    /// Tests the Day 06 Part 1 solver method against the actual problem solution.
    #[test]
    fn test_day06_part1_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part1(&input);
        assert_eq!(7864, solution);
    }

    /// Tests the Day 06 Part 2 solver method against the actual problem solution.
    #[test]
    fn test_day06_part2_actual() {
        let input = process_input_file(PROBLEM_INPUT_FILE);
        let solution = solve_part2(&input);
        assert_eq!(1695, solution);
    }
}