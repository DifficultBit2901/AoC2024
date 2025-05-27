use std::{env, error::Error, fs};

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

fn stage_one(in_file: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(in_file)?.to_lowercase();
    let lines = content.lines();
    let mut count = 0;
    for (y, line) in lines.enumerate() {
        let chars = line.chars();
        for (x, char) in chars.enumerate() {
            if char == 'x' {
                let x_i = x as isize;
                let y_i = y as isize;
                // test ->
                if test_char('m', x_i + 1, y_i, &content)
                    && test_char('a', x_i + 2, y_i, &content)
                    && test_char('s', x_i + 3, y_i, &content)
                {
                    count += 1;
                }
                // test <-
                if test_char('m', x_i - 1, y_i, &content)
                    && test_char('a', x_i - 2, y_i, &content)
                    && test_char('s', x_i - 3, y_i, &content)
                {
                    count += 1;
                }
                // test v
                if test_char('m', x_i, y_i + 1, &content)
                    && test_char('a', x_i, y_i + 2, &content)
                    && test_char('s', x_i, y_i + 3, &content)
                {
                    count += 1;
                }
                // test ^
                if test_char('m', x_i, y_i - 1, &content)
                    && test_char('a', x_i, y_i - 2, &content)
                    && test_char('s', x_i, y_i - 3, &content)
                {
                    count += 1;
                }
                // test dr
                if test_char('m', x_i + 1, y_i + 1, &content)
                    && test_char('a', x_i + 2, y_i + 2, &content)
                    && test_char('s', x_i + 3, y_i + 3, &content)
                {
                    count += 1;
                }
                // test ur
                if test_char('m', x_i + 1, y_i - 1, &content)
                    && test_char('a', x_i + 2, y_i - 2, &content)
                    && test_char('s', x_i + 3, y_i - 3, &content)
                {
                    count += 1;
                }
                // test dl
                if test_char('m', x_i - 1, y_i + 1, &content)
                    && test_char('a', x_i - 2, y_i + 2, &content)
                    && test_char('s', x_i - 3, y_i + 3, &content)
                {
                    count += 1;
                }
                // test ul
                if test_char('m', x_i - 1, y_i - 1, &content)
                    && test_char('a', x_i - 2, y_i - 2, &content)
                    && test_char('s', x_i - 3, y_i - 3, &content)
                {
                    count += 1;
                }
            }
        }
    }
    Ok(format!("{count}"))
}

fn stage_two(in_file: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(in_file)?.to_lowercase();
    let lines = content.lines();
    let mut count = 0;
    for (y, line) in lines.enumerate() {
        let chars = line.chars();
        for (x, char) in chars.enumerate() {
            if char == 'a' {
                let x_i = x as isize;
                let y_i = y as isize;
                let ul_dr = (test_char('m', x_i - 1, y_i - 1, &content)
                    && test_char('s', x_i + 1, y_i + 1, &content))
                    || (test_char('s', x_i - 1, y_i - 1, &content)
                        && test_char('m', x_i + 1, y_i + 1, &content));
                if !ul_dr {
                    continue;
                }
                let ur_dl = (test_char('m', x_i + 1, y_i - 1, &content)
                    && test_char('s', x_i - 1, y_i + 1, &content))
                    || (test_char('s', x_i + 1, y_i - 1, &content)
                        && test_char('m', x_i - 1, y_i + 1, &content));
                if ur_dl {
                    count += 1;
                }
            }
        }
    }
    Ok(format!("{count}"))
}

fn test_char(c: char, x: isize, y: isize, content: &str) -> bool {
    if x < 0 || y < 0 {
        return false;
    }
    let x = x as usize;
    let y = y as usize;
    let lines: Vec<&str> = content.lines().collect();
    match lines.get(y) {
        None => false,
        Some(line) => {
            let chars: Vec<char> = line.chars().collect();
            match chars.get(x) {
                None => false,
                Some(ch) => *ch == c,
            }
        }
    }
}
