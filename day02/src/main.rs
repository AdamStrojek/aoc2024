use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    solve_part1("example.txt")?;
    solve_part1("input.txt")?;

    solve_part2("example.txt")?;
    solve_part2("input.txt")?;

    Ok(())
}

fn solve_part1(file_name: &str) -> Result<u32> {
    println!("Solving file {file_name}");

    let content = fs::read_to_string(file_name)?;

    let mut result = 0;

    for line in content.lines() {
        // 7 6 4 2 1
        let numbers: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();

        if check_line(numbers) {
            result += 1;
        }
    }

    println!("Solution for file {file_name}: {result}");

    Ok(result)
}

fn solve_part2(file_name: &str) -> Result<u32> {
    println!("Solving file {file_name}");

    let content = fs::read_to_string(file_name)?;

    let mut result = 0;

    for line in content.lines() {
        // 7 6 4 2 1
        let numbers: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();

        if check_line(numbers.clone()) {
            result += 1;
        } else {
            for i in 0..numbers.len() {
                let mut numbers = numbers.clone();
                numbers.remove(i);

                if check_line(numbers.clone()) {
                    result += 1;
                    break;
                }
            }
        }
    }

    println!("Solution for file {file_name}: {result}");

    Ok(result)
}

fn check_line(numbers: Vec<i32>) -> bool {
    let mut it = numbers.iter().peekable();

    let mut prev_sign = 0;

    while let Some(i) = it.next() {
        if let Some(&j) = it.peek() {
            let diff = j - i;
            let sign = diff.signum();
            let diff = diff.abs();

            if (prev_sign != 0 && prev_sign != sign) || diff < 1 || diff > 3 {
                return false;
            }
            prev_sign = sign;
        } else {
            return true;
        }
    }

    true
}
