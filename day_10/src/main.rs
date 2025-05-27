use std::{collections::HashSet, env, fs, u8};

use data::Node;

mod data;

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
    let map = generate_map(in_file)?;
    let heads = get_trail_heads(&map);
    let total_score = heads
        .into_iter()
        .map(|head| get_head_score(head, &map))
        .sum::<usize>();
    Ok(format!("{total_score}"))
}

fn stage_two(in_file: &str) -> Result<String> {
    let map = generate_map(in_file)?;
    let heads = get_trail_heads(&map);
    let total_score = heads
        .into_iter()
        .map(|head| get_head_rating(head, &map))
        .sum::<usize>();
    Ok(format!("{total_score}"))
}

fn get_head_score(head: Node, map: &Vec<Vec<u8>>) -> usize {
    let mut active_elements = vec![head];
    let mut peaks = HashSet::new();

    for _ in 0..9 {
        if active_elements.len() == 0 {
            return 0;
        }
        let mut new_elements = Vec::new();
        for elem in active_elements {
            for neighbor in elem.get_valid_neighbors(&map) {
                new_elements.push(neighbor);
            }
        }
        active_elements = new_elements;
    }

    for elem in active_elements {
        peaks.insert(elem);
    }
    peaks.len()
}

fn get_head_rating(head: Node, map: &Vec<Vec<u8>>) -> usize {
    let mut active_elements = vec![head];
    let mut peaks = Vec::new();

    for _ in 0..9 {
        if active_elements.len() == 0 {
            return 0;
        }
        let mut new_elements = Vec::new();
        for elem in active_elements {
            for neighbor in elem.get_valid_neighbors(&map) {
                new_elements.push(neighbor);
            }
        }
        active_elements = new_elements;
    }

    for elem in active_elements {
        peaks.push(elem);
    }
    peaks.len()
}

fn generate_map(in_file: &str) -> Result<Vec<Vec<u8>>> {
    let content = fs::read_to_string(in_file)?;
    let map = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_string())
                .map(|ch_str| u8::from_str_radix(&ch_str, 10))
                .map(|res| res.unwrap_or(0))
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();
    Ok(map)
}

fn get_trail_heads(map: &Vec<Vec<u8>>) -> Vec<Node> {
    let mut heads = Vec::new();
    for (y, col) in map.iter().enumerate() {
        for (x, cell) in col.iter().enumerate() {
            if *cell == 0 {
                heads.push(Node::new(0, x, y));
            }
        }
    }
    heads
}
