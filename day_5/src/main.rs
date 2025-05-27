use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

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
    let file = File::open(in_file)?;
    let mut buf_reader = BufReader::new(file);
    let page_rules = generate_rules(&mut buf_reader)?;
    let updates = generate_updates(&mut buf_reader)?;
    let valid_updates: u32 = updates
        .into_iter()
        .filter(|update| is_update_valid(&update, &page_rules))
        .map(|update| {
            let pages: Vec<&str> = update.split(',').collect();
            let middle = &pages[pages.len() / 2];
            u32::from_str_radix(middle, 10).unwrap_or(0)
        })
        .sum();
    Ok(format!("{valid_updates}"))
}

fn stage_two(in_file: &str) -> Result<String> {
    let file = File::open(in_file)?;
    let mut buf_reader = BufReader::new(file);
    let page_rules = generate_rules(&mut buf_reader)?;
    let updates = generate_updates(&mut buf_reader)?;
    let invalid_updates: u32 = updates
        .into_iter()
        .filter(|update| !is_update_valid(&update, &page_rules))
        .map(|update| update.split(',').map(|page| page.to_string()).collect())
        .map(|update: Vec<String>| {
            let mut update: Vec<String> = update.clone();
            let mut i = 0;
            while i < update.len() {
                if let Some(rules) = page_rules.get(&update[i]) {
                    for j in 0..i {
                        if rules.contains(&update[j]) {
                            let tmp = update[i].clone();
                            update[i] = update[i - 1].clone();
                            update[i - 1] = tmp;
                            i = 0;
                            break;
                        }
                    }
                }
                i += 1;
            }
            update
        })
        .map(|update| {
            let middle = &update[update.len() / 2];
            u32::from_str_radix(middle, 10).unwrap_or(0)
        })
        .sum();
    Ok(format!("{invalid_updates}"))
}

fn generate_rules(buf_reader: &mut BufReader<File>) -> Result<HashMap<String, Vec<String>>> {
    let mut page_rules: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut buf = String::new();
        buf_reader.read_line(&mut buf)?;
        let line = buf.trim();
        if line.is_empty() {
            break;
        }
        let (earlier_page, later_page) = line.split_once('|').unwrap();
        let updated_rules_for_page = match page_rules.get(earlier_page) {
            None => vec![later_page.to_string()],
            Some(rules) => {
                let mut updated_rulees = rules.clone();
                updated_rulees.push(later_page.to_string());
                updated_rulees
            }
        };
        page_rules.insert(earlier_page.to_string(), updated_rules_for_page);
    }
    Ok(page_rules)
}

fn generate_updates(buf_reader: &mut BufReader<File>) -> Result<Vec<String>> {
    let mut updates = Vec::new();
    loop {
        let mut buf = String::new();
        let len = buf_reader.read_line(&mut buf)?;
        if len == 0 {
            break;
        }
        let update = buf.trim();
        updates.push(update.to_string());
    }
    Ok(updates)
}

fn is_update_valid(line: &str, rules: &HashMap<String, Vec<String>>) -> bool {
    let pages: Vec<&str> = line.split(',').collect();
    for (i, page) in pages.iter().enumerate() {
        match rules.get(*page) {
            None => continue,
            Some(page_rules) => {
                if page_rules.iter().any(|rule| {
                    for j in 0..i {
                        if pages[j] == rule {
                            return true;
                        }
                    }
                    false
                }) {
                    return false;
                }
            }
        }
    }
    true
}
