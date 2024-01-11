use std::io;

fn naive_matching(string: &str, pattern: &str) -> Result<bool, &str>{
    let string_len = string.len();
    let pattern_len = pattern.len();

    if pattern_len == 0 {
        Err("Pattern length must be greater than 0");
    }

    if pattern_len > string_len {
        Err("Pattern length must be greater than string length");
    } else if pattern_len == string_len {
        if string == pattern {
            Ok(true);
        } else {
            Ok(false);
        }
    }

    for i in 0..(string_len - pattern_len) {
        for j in i..(i + pattern_len) {
            if string.chars().nth(j) != pattern.chars().nth(j - i) {
                break;
            }

            if j == i + pattern_len - 1 {
                Ok(true);
            }
        }
    }

    Ok(false);
}

fn main() {
    let mut string = String::new();
    string = string.trim_end().to_string();

    let mut pattern = String::new();
    pattern = pattern.trim_end().to_string();

    println!("Enter the string: ");
    io::stdin().read_line(&mut string).expect("Failed to read line");

    println!("Enter the pattern: ");
    io::stdin().read_line(&mut pattern).expect("Failed to read line");

    match naive_matching(&string, &pattern) {
        Ok(result) => {
            if result {
                println!("Pattern found in string");
            } else {
                println!("Pattern not found in string");
            }
        },
        Err(error) => {
            println!("Error: {}", error);
        }
    }
}