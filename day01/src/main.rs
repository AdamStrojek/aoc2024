use anyhow::{Context, Result};
use std::fs;

fn main() -> Result<()> {
    solve("example.txt")?;

    Ok(())
}

fn solve(file_name: &str) -> Result<u32> {
    println!("Processing file {file_name}");

    let binding = fs::read_to_string(file_name)?;
    let lines = binding.lines();

    let mut list_a: Vec<u32> = vec![];
    let mut list_b: Vec<u32> = vec![];

    for line in lines {
        let mut it = line.split_whitespace();
        let a = it.next().context("Missing a in line")?.parse()?;
        let b = it.next().context("Missing b in line")?.parse()?;

        list_a.push(a);
        list_b.push(b);
    }

    list_a.sort();
    list_b.sort();

    let distance = list_a.into_iter().zip(list_b).map(|(a, b)| a.abs_diff(b)).sum();

    println!("Answer: {distance}");

    Ok(distance)
}
