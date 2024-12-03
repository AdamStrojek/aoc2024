use std::fs;
use regex::Regex;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    solve("example.txt")?;
    solve("input.txt")?;

    Ok(())
}

fn solve(file_name: &str) -> Result<i32> {
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
