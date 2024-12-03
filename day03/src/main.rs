use std::fs;
use regex::Regex;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    solve_part1("example.txt")?;
    solve_part1("input.txt")?;

    solve_part2("example2.txt")?;
    solve_part2("input.txt")?;

    Ok(())
}

fn solve_part1(file_name: &str) -> Result<i32> {
    let content = fs::read_to_string(file_name)?;

    let re = Regex::new(r"mul\((\d+),(\d+)\)")?;

    let mut result = 0;

    for capture in re.captures_iter(&content) {
        let a: i32 = capture.get(1).context("Can't get group")?.as_str().parse()?;
        let b: i32 = capture.get(2).context("Can't get group")?.as_str().parse()?;
        result += a * b;
    }

    println!("Result: {result}");

    Ok(result)
}


fn solve_part2(file_name: &str) -> Result<i32> {
    let content = fs::read_to_string(file_name)?;

    let re = Regex::new(r"(mul|do|don't)\(((\d+),(\d+))?\)")?;

    let mut result = 0;
    let mut disable = false;

    for capture in re.captures_iter(&content) {
        let x = capture.get(1).context("Can't get operation")?.as_str();
        match x {
            "mul" => if !disable {
                if let (Some(a), Some(b)) = (capture.get(3).and_then(|x| Some(x.as_str())), capture.get(4).and_then(|x| Some(x.as_str()))) {
                    result += a.parse::<i32>()? * b.parse::<i32>()?;
                }
            },
            "do" => if capture.get(2).is_none() {
                disable = false;
            }
            "don't" => if capture.get(2).is_none() {
                disable = true
            },
            _ => {panic!("Ignoring {x}")},
        };
    }

    println!("Result: {result}");

    Ok(result)
}