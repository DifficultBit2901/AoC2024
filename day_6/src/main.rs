use std::{env, fs};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file> [stage]", args[0]);
        return;
    }
    let in_file = args.get(1).unwrap();
    let stage = match args.get(2) {
        None => "1",
        Some(s) => &s,
    };
    let res = match stage {
        "1" => stage_one(in_file),
        "2" => stage_two(in_file),
        _ => {
            eprintln!("Unknown Stage");
            Ok("".to_string())
        }
    };
    match res {
        Err(e) => eprintln!("Error: {}", e),
        Ok(r) => {
            if !r.is_empty() {
                println!("Result: {}", r);
            }
        }
    }
}

fn stage_one(in_file: &str) -> Result<String> {
    let content = fs::read_to_string(in_file)?;
    let mut grid = content
        .lines()
        .map(str::trim)
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut velocity: (i8, i8) = (0, -1);
    let mut pos = {
        let mut pos = (0, 0);
        for (y, line) in grid.iter().enumerate() {
            for (x, ch) in line.iter().enumerate() {
                if *ch == '^' {
                    pos = (x, y);
                    break;
                }
            }
        }
        pos
    };
    let mut count = 0;

    loop {
        if grid[pos.1][pos.0] != 'X' {
            count += 1;
        }
        grid[pos.1][pos.0] = 'X';
        let new_y = pos.1 as isize + velocity.1 as isize;
        let new_x = pos.0 as isize + velocity.0 as isize;
        if new_y < 0 || new_y as usize >= grid.len() {
            break;
        }
        if new_x < 0 || new_x as usize >= grid[new_y as usize].len() {
            break;
        }
        if grid[new_y as usize][new_x as usize] == '#' {
            velocity = (-velocity.1, velocity.0);
            continue;
        }
        pos.0 = new_x as usize;
        pos.1 = new_y as usize;
    }

    println!(
        "{}",
        grid.iter()
            .map(|line_chars| line_chars
                .iter()
                .map(char::to_string)
                .collect::<Vec<String>>()
                .join(""))
            .collect::<Vec<String>>()
            .join("\n")
    );
    Ok(format!("{count}"))
}

fn stage_two(in_file: &str) -> Result<String> {
    let content = fs::read_to_string(in_file)?;
    let mut grid = content
        .lines()
        .map(str::trim)
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut velocity: (i8, i8) = (0, -1);
    let mut pos = {
        let mut pos = (0, 0);
        for (y, line) in grid.iter().enumerate() {
            for (x, ch) in line.iter().enumerate() {
                if *ch == '^' {
                    pos = (x, y);
                    break;
                }
            }
        }
        pos
    };
    let initial_pos = pos.clone();

    loop {
        grid[pos.1][pos.0] = 'X';
        let new_y = pos.1 as isize + velocity.1 as isize;
        let new_x = pos.0 as isize + velocity.0 as isize;
        if new_y < 0 || new_y as usize >= grid.len() {
            break;
        }
        if new_x < 0 || new_x as usize >= grid[new_y as usize].len() {
            break;
        }
        if grid[new_y as usize][new_x as usize] == '#' {
            velocity = (-velocity.1, velocity.0);
            continue;
        }
        pos.0 = new_x as usize;
        pos.1 = new_y as usize;
    }

    let mut count = 0;

    for (y, line) in grid.iter().enumerate() {
        for (x, ch) in line.iter().enumerate() {
            if *ch == 'X' {
                let mut grid_copy = grid.clone();
                grid_copy[y][x] = '0';
                if is_loop(grid_copy, &initial_pos) {
                    count += 1;
                }
            }
        }
    }

    Ok(format!("{count}"))
}

fn print_grid(grid: &Vec<Vec<char>>) {
    println!(
        "{}",
        grid.iter()
            .map(|line| line
                .iter()
                .map(|ch| ch.to_string())
                .collect::<Vec<String>>()
                .join(""))
            .collect::<Vec<String>>()
            .join("\n")
    );
}

fn is_loop(mut grid: Vec<Vec<char>>, pos: &(usize, usize)) -> bool {
    let mut pos = pos.clone();
    let mut velocity = (0, -1);
    let mut hit_before = false;
    loop {
        let cur_char = &grid[pos.1][pos.0];
        if *cur_char == '+' {
            if hit_before {
                return true;
            }
            hit_before = true;
        } else if (*cur_char == '|' && velocity.0 != 0) || (*cur_char == '-' && velocity.1 != 0) {
            grid[pos.1][pos.0] = '+';
        } else if velocity.0 != 0 {
            grid[pos.1][pos.0] = '-';
        } else {
            grid[pos.1][pos.0] = '|';
        }
        let new_y = pos.1 as isize + velocity.1 as isize;
        let new_x = pos.0 as isize + velocity.0 as isize;
        if new_y < 0 || new_y as usize >= grid.len() {
            break;
        }
        if new_x < 0 || new_x as usize >= grid[new_y as usize].len() {
            break;
        }
        let new_char = &grid[new_y as usize][new_x as usize];
        if *new_char == '#' || *new_char == '0' {
            velocity = (-velocity.1, velocity.0);
            continue;
        }
        pos.0 = new_x as usize;
        pos.1 = new_y as usize;
        hit_before = false;
    }
    return false;
}
