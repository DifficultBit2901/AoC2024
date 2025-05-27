use std::{env, fs, usize};

use data::{StorageCell, StorageFile};

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
    let mut storage = generate_old_storage(in_file)?;
    sort_storage(&mut storage);
    let sum: usize = storage
        .iter()
        .enumerate()
        .map(|(index, item)| item.get_content_valued(index))
        .sum();
    Ok(format!("{sum}"))
}

fn stage_two(in_file: &str) -> Result<String> {
    let mut storage = generate_compact_storage(in_file)?;
    sort_compact_storage(&mut storage);
    let sum: usize = storage
        .iter()
        .map(StorageFile::flatten)
        .flatten()
        .enumerate()
        .map(|(index, item)| item.get_content_valued(index))
        .sum();
    Ok(format!("{sum}"))
}

fn generate_compact_storage(in_file: &str) -> Result<Vec<StorageFile>> {
    let content = fs::read_to_string(in_file)?;
    let mut storage = Vec::new();
    let mut file_index = 0;
    let mut is_file = true;
    let mut was_empty_file = false;
    for ch in content.chars() {
        let count = usize::from_str_radix(&ch.to_string(), 10).unwrap_or(0);
        let file = if was_empty_file {
            let prev_empty_thing: StorageFile = storage.pop().unwrap();
            StorageFile::empty(count + prev_empty_thing.get_size())
        } else if is_file {
            let file = StorageFile::full(count, file_index);
            file_index += 1;
            file
        } else {
            StorageFile::empty(count)
        };
        was_empty_file = false;
        is_file = !is_file;
        if count == 0 {
            if !is_file {
                was_empty_file = true;
            }
            continue;
        }
        storage.push(file);
    }
    Ok(storage)
}

fn generate_old_storage(in_file: &str) -> Result<Vec<StorageCell>> {
    let content = fs::read_to_string(in_file)?;
    let mut storage = Vec::new();
    let mut file_index = 0;
    let mut is_file = true;
    for ch in content.chars() {
        let count = usize::from_str_radix(&ch.to_string(), 10).unwrap_or(0);
        let cell = if is_file {
            let cell = StorageCell::new(file_index);
            file_index += 1;
            cell
        } else {
            StorageCell::Empty
        };
        is_file = !is_file;
        if count == 0 {
            continue;
        }
        for _ in 0..count {
            storage.push(cell.clone());
        }
    }
    Ok(storage)
}

fn sort_storage(storage: &mut Vec<StorageCell>) {
    let mut empty_ptr = 0;
    let mut full_ptr = storage.len() - 1;
    while full_ptr > empty_ptr {
        while !storage[empty_ptr].is_empty() && full_ptr > empty_ptr {
            empty_ptr += 1;
        }
        while storage[full_ptr].is_empty() && full_ptr > empty_ptr {
            full_ptr -= 1;
        }
        let tmp = storage[empty_ptr].clone();
        storage[empty_ptr] = storage[full_ptr].clone();
        storage[full_ptr] = tmp;
    }
}

fn sort_compact_storage(storage: &mut Vec<StorageFile>) {
    let mut full_ptr = storage.len() - 1;
    let mut last_swapped_value = usize::MAX;
    while full_ptr > 0 {
        while full_ptr > 0 && storage[full_ptr].is_empty() {
            full_ptr -= 1;
        }
        // test if there's an empty slot for the file
        let full_file = storage[full_ptr].clone();
        if full_file.get_value() >= last_swapped_value {
            full_ptr -= 1;
            continue;
        }
        // println!("Try swapping {:#?}", &full_file);
        for empty_ptr in 0..full_ptr {
            if !storage[empty_ptr].can_fit(full_file.get_size()) {
                let tried_file = &storage[empty_ptr];
                match tried_file {
                    StorageFile::Empty { .. } => {
                        // println!("Couldn't fit in {:#?}, at pos {}", tried_file, empty_ptr)
                    }
                    _ => {}
                }
                continue;
            }
            // println!("Swapped {:#?}", &full_file);
            last_swapped_value = full_file.get_value();
            let (swapped, remaining) = storage[empty_ptr].split_off(full_file.get_size());
            storage[full_ptr] = swapped;
            storage[empty_ptr] = full_file;
            if remaining.get_size() > 0 {
                storage.insert(empty_ptr + 1, remaining);
            }
            break;
        }
        full_ptr -= 1;
    }
}
