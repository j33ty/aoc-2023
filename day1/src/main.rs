use std::{env, fs};
use std::collections::HashMap;

// Run: cargo run -- src/input1.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let mut digit_english = HashMap::new();
            digit_english.insert("one", 1u32);
            digit_english.insert("two", 2u32);
            digit_english.insert("three", 3u32);
            digit_english.insert("four", 4u32);
            digit_english.insert("five", 5u32);
            digit_english.insert("six", 6u32);
            digit_english.insert("seven", 7u32);
            digit_english.insert("eight", 8u32);
            digit_english.insert("nine", 9u32);

            let file_path = &args[1];
            let file = fs::read_to_string(file_path).
                expect("failed to read file");
            let lines = file.lines();

            let mut sum: u32 = 0;
            lines.for_each(|line| {
                let (mut first_digit, mut last_digit): (Option<(u32, u32)>, Option<(u32, u32)>) = (None, None);
                for (i, c) in line.chars().enumerate() {
                    if c.is_digit(10) {
                        if first_digit.is_none() {
                            first_digit = Some((c.to_digit(10).unwrap(), i as u32));
                        }
                        last_digit = Some((c.to_digit(10).unwrap(), i as u32));
                    }
                }

                for (digit_name, digit_val) in &digit_english {
                    let first_instance = line.find(digit_name);
                    if first_instance.is_some() && (first_digit.is_none() || first_digit.unzip().1.unwrap() > first_instance.unwrap() as u32) {
                        first_digit = Option::from((digit_val.clone(), first_instance.unwrap() as u32))
                    }

                    let last_instance = line.rfind(digit_name);
                    if last_instance.is_some() && (last_digit.is_none() || last_digit.unzip().1.unwrap() < last_instance.unwrap() as u32) {
                        last_digit = Option::from((digit_val.clone(), last_instance.unwrap() as u32))
                    }
                }

                if first_digit.is_some() && last_digit.is_some() {
                    sum += 10 * first_digit.unwrap().0 + last_digit.unwrap().0
                } else if first_digit.is_some() && last_digit.is_none() {
                    sum += first_digit.unwrap().0
                }
            });
            println!("{sum}");
        }
        _ => {
            eprintln!("invalid arg length, should be 2");
            return;
        }
    }
}
