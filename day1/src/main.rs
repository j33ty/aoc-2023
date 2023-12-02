use std::{env, fs};

// Run: cargo run -- src/input1.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let file_path = &args[1];
            let file = fs::read_to_string(file_path).
                expect("failed to read file");
            let lines = file.lines();

            let mut sum: u32 = 0;
            lines.for_each(|line| {
                let (mut first_digit, mut last_digit): (Option<u32>, Option<u32>) = (None, None);
                for c in line.chars() {
                    if c.is_digit(10) {
                        if first_digit.is_none() {
                            first_digit = Some(c.to_digit(10).unwrap());
                        }
                        last_digit = Some(c.to_digit(10).unwrap());
                    }
                }

                if first_digit.is_some() && last_digit.is_some() {
                    sum += 10 * first_digit.unwrap() + last_digit.unwrap()
                } else if first_digit.is_some() && last_digit.is_none() {
                    sum += first_digit.unwrap()
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
