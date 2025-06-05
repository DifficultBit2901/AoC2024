use std::{collections::HashMap, fs};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn generate_map(in_file: &str) -> Result<Vec<Vec<char>>> {
    let content = fs::read_to_string(in_file)?;
    Ok(content
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>())
}

pub fn regionize_map(map: &Vec<Vec<char>>) -> HashMap<(usize, usize), usize> {
    let mut region_idx = 0;
    let mut region_map = HashMap::new();
    for (y, line) in map.iter().enumerate() {
        for (x, _) in line.iter().enumerate() {
            if !region_map.contains_key(&(x, y)) {
                try_insert_region_part(region_idx, (x, y), map, &mut region_map);
                region_idx += 1;
            }
        }
    }
    region_map
}

fn try_insert_region_part(
    index: usize,
    pos: (usize, usize),
    map: &Vec<Vec<char>>,
    region_map: &mut HashMap<(usize, usize), usize>,
) {
    if region_map.contains_key(&pos) {
        return;
    }
    let (x, y) = pos;
    region_map.insert((x, y), index);
    if y > 0 && map[y][x] == map[y - 1][x] {
        try_insert_region_part(index, (x, y - 1), map, region_map);
    }
    if y < map.len() - 1 && map[y][x] == map[y + 1][x] {
        try_insert_region_part(index, (x, y + 1), map, region_map);
    }
    if x > 0 && map[y][x] == map[y][x - 1] {
        try_insert_region_part(index, (x - 1, y), map, region_map);
    }
    if x < map[y].len() - 1 && map[y][x] == map[y][x + 1] {
        try_insert_region_part(index, (x + 1, y), map, region_map);
    }
}
