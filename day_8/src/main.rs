use std::{
    collections::{HashMap, HashSet},
    env, fs,
    str::Chars,
};

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
    let map = generate_map(&content);
    let dimensions = map.get(&'.').unwrap()[0];
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for (k, antennas) in map.iter() {
        if *k == '.' {
            continue;
        }
        for (i, ant_a) in antennas.iter().enumerate() {
            for (j, ant_b) in antennas.iter().enumerate() {
                if j <= i {
                    continue;
                }
                let signed_delta_x = ant_a.0 as isize - ant_b.0 as isize;
                let signed_delta_y = ant_a.1 as isize - ant_b.1 as isize;

                let potential1 = (
                    ant_a.0 as isize + signed_delta_x,
                    ant_a.1 as isize + signed_delta_y,
                );
                if potential1.0 >= 0
                    && potential1.0 < dimensions.0 as isize
                    && potential1.1 >= 0
                    && potential1.1 < dimensions.1 as isize
                {
                    antinodes.insert((potential1.0 as usize, potential1.1 as usize));
                }
                let potential2 = (
                    ant_b.0 as isize - signed_delta_x,
                    ant_b.1 as isize - signed_delta_y,
                );
                if potential2.0 >= 0
                    && potential2.0 < dimensions.0 as isize
                    && potential2.1 >= 0
                    && potential2.1 < dimensions.1 as isize
                {
                    antinodes.insert((potential2.0 as usize, potential2.1 as usize));
                }
            }
        }
    }
    Ok(format!("{}", antinodes.iter().count()))
}

fn stage_two(in_file: &str) -> Result<String> {
    let content = fs::read_to_string(in_file)?;
    let map = generate_map(&content);
    let dimensions = map.get(&'.').unwrap()[0];
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();
    for (k, antennas) in map.iter() {
        if *k == '.' {
            continue;
        }
        for (i, ant_a) in antennas.iter().enumerate() {
            for (j, ant_b) in antennas.iter().enumerate() {
                if j <= i {
                    continue;
                }
                let signed_delta_x = ant_a.0 as isize - ant_b.0 as isize;
                let signed_delta_y = ant_a.1 as isize - ant_b.1 as isize;

                let mut offset = 0;
                loop {
                    let potential1 = (
                        ant_a.0 as isize + signed_delta_x * offset,
                        ant_a.1 as isize + signed_delta_y * offset,
                    );
                    if potential1.0 >= 0
                        && potential1.0 < dimensions.0 as isize
                        && potential1.1 >= 0
                        && potential1.1 < dimensions.1 as isize
                    {
                        antinodes.insert((potential1.0 as usize, potential1.1 as usize));
                    } else {
                        break;
                    }
                    offset += 1;
                }
                offset = 0;
                loop {
                    let potential2 = (
                        ant_b.0 as isize - signed_delta_x * offset,
                        ant_b.1 as isize - signed_delta_y * offset,
                    );
                    if potential2.0 >= 0
                        && potential2.0 < dimensions.0 as isize
                        && potential2.1 >= 0
                        && potential2.1 < dimensions.1 as isize
                    {
                        antinodes.insert((potential2.0 as usize, potential2.1 as usize));
                    } else {
                        break;
                    }
                    offset += 1;
                }
            }
        }
    }
    Ok(format!("{}", antinodes.iter().count()))
}

fn generate_map(content: &str) -> HashMap<char, Vec<(usize, usize)>> {
    let mut map: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (y, line) in content.lines().map(str::trim).enumerate() {
        map.insert('.', vec![(line.len(), content.lines().count())]);
        for (x, ch) in line.chars().enumerate() {
            if ch == '.' {
                continue;
            }
            let mut positions: Vec<(usize, usize)> = match map.get(&ch) {
                Some(pos) => pos.clone(),
                None => Vec::new(),
            };
            positions.push((x, y));
            map.insert(ch, positions);
        }
    }

    map
}
