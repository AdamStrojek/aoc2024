use std::fs;
use anyhow::{Context, Result};
use grid::{grid, Grid};

fn main() -> Result<()> {
    solve("example.txt")?;
    solve("input.txt")?;

    Ok(())
}

fn solve(file_name: &str) -> Result<u32> {
    println!("Processing file: {file_name}");

    let data = load_data(file_name)?;
    let mut result = 0;

    dbg!(&data);

    for ((row, col), _) in data.indexed_iter() {
        for dir_row in [-1, 0, 1] {
            for dir_col in [-1, 0, 1] {
                if check_word(&data, row, col, dir_row, dir_col) {
                    result += 1;
                    println!("Found: {row}x{col}");
                }
            }
        }
    }

    println!("Result: {result}");

    Ok(result)
}

fn check_word(data: &Grid<char>, pos_row: usize, pos_col: usize, dir_row: isize, dir_col: isize) -> bool {
    for (pos, ch) in "XMAS".char_indices() {
        let data_ch = data.get(
            pos_row.wrapping_add_signed(dir_row * pos as isize),
            pos_col.wrapping_add_signed(dir_col * pos as isize)
        );

        if data_ch.is_none_or(|data_ch| *data_ch != ch) {
            return false;
        }
    }
    true
}

fn load_data(file_name: &str) -> Result<Grid<char>> {
    let content = fs::read_to_string(file_name)?;
    let line_width = content.find(char::is_whitespace).context("Malformed file!")?;
    let content = content.chars().filter(|ch| !ch.is_whitespace()).collect();

    let grid = Grid::from_vec(content, line_width);
    Ok(grid)
}
