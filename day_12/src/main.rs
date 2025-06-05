use std::{
    collections::{BTreeSet, HashSet},
    env,
};

mod proc;
use proc::*;

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
    let regions = regionize_map(&map);
    let region_set = regions
        .values()
        .map(|reg| *reg)
        .collect::<BTreeSet<usize>>();
    let mut total_cost = 0;
    for reg_idx in region_set {
        let region_fields = regions
            .iter()
            .filter(|(_, region)| **region == reg_idx)
            .map(|((x, y), _)| (*x, *y))
            .collect::<Vec<(usize, usize)>>();
        let area = region_fields.len();
        let mut perimeter = 0;
        for (x, y) in region_fields.iter() {
            let mut positions = vec![(x + 1, *y), (*x, y + 1)];
            if *x > 0 {
                positions.push((x - 1, *y));
            } else {
                perimeter += 1;
            }
            if *y > 0 {
                positions.push((*x, y - 1));
            } else {
                perimeter += 1;
            }
            for pos in positions.iter() {
                if !regions.contains_key(pos)
                    || regions.get(pos).unwrap() != regions.get(&(*x, *y)).unwrap()
                {
                    perimeter += 1;
                }
            }
        }
        total_cost += perimeter * area;
    }
    Ok(format!("{total_cost}"))
}

fn stage_two(in_file: &str) -> Result<String> {
    let map = generate_map(in_file)?;
    let regions = regionize_map(&map);
    let region_set = regions
        .values()
        .map(|reg| *reg)
        .collect::<BTreeSet<usize>>();
    let mut total_cost = 0;
    for reg_idx in region_set {
        let mut region_fields = regions
            .iter()
            .filter(|(_, region)| **region == reg_idx)
            .map(|((x, y), _)| (*x, *y))
            .collect::<Vec<(usize, usize)>>();
        region_fields.sort_by(|a, b| {
            if a.1 == b.1 {
                return a.0.cmp(&b.0);
            }
            a.1.cmp(&b.1)
        });
        let area = region_fields.len();
        let mut sides = 0;
        let mut west_sides = HashSet::new();
        let mut east_sides = HashSet::new();
        let mut north_sides = HashSet::new();
        let mut south_sides = HashSet::new();
        for (x, y) in region_fields.iter() {
            // West Side
            if *x > 0 {
                let west_side = (x - 1, *y);
                if regions.get(&(*x, *y)).unwrap() != regions.get(&west_side).unwrap() {
                    west_sides.insert((*x, *y));
                    if !west_sides.contains(&(*x, y + 1))
                        && (*y == 0 || !west_sides.contains(&(*x, y - 1)))
                    {
                        sides += 1;
                    }
                }
            } else {
                west_sides.insert((*x, *y));
                if !west_sides.contains(&(*x, y + 1))
                    && (*y == 0 || !west_sides.contains(&(*x, y - 1)))
                {
                    sides += 1;
                }
            }
            // East Side
            let east_side = (x + 1, *y);
            if regions.contains_key(&east_side) {
                if regions.get(&(*x, *y)).unwrap() != regions.get(&east_side).unwrap() {
                    east_sides.insert((*x, *y));
                    if !east_sides.contains(&(*x, *y + 1))
                        && (*y == 0 || !east_sides.contains(&(*x, y - 1)))
                    {
                        sides += 1;
                    }
                }
            } else {
                east_sides.insert((*x, *y));
                if !east_sides.contains(&(*x, *y + 1))
                    && (*y == 0 || !east_sides.contains(&(*x, y - 1)))
                {
                    sides += 1;
                }
            }
            // North side
            if *y > 0 {
                let north_side = (*x, y - 1);
                if regions.get(&(*x, *y)).unwrap() != regions.get(&north_side).unwrap() {
                    north_sides.insert((*x, *y));
                    if !north_sides.contains(&(x + 1, *y))
                        && (*x == 0 || !north_sides.contains(&(x - 1, *y)))
                    {
                        sides += 1;
                    }
                }
            } else {
                north_sides.insert((*x, *y));
                if !north_sides.contains(&(x + 1, *y))
                    && (*x == 0 || !north_sides.contains(&(x - 1, *y)))
                {
                    sides += 1;
                }
            }
            // South Side
            let south_side = (*x, y + 1);
            if regions.contains_key(&south_side) {
                if regions.get(&(*x, *y)).unwrap() != regions.get(&south_side).unwrap() {
                    south_sides.insert((*x, *y));
                    if !south_sides.contains(&(x + 1, *y))
                        && (*x == 0 || !south_sides.contains(&(x - 1, *y)))
                    {
                        sides += 1;
                    }
                }
            } else {
                south_sides.insert((*x, *y));
                if !south_sides.contains(&(x + 1, *y))
                    && (*x == 0 || !south_sides.contains(&(x - 1, *y)))
                {
                    sides += 1;
                }
            }
        }
        total_cost += sides * area;
    }
    Ok(format!("{total_cost}"))
}
