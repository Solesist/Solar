use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }

    let filename = &args[1];

    let mut lines = match fs::read_to_string(filename) {
        Ok(content) => content.lines().map(String::from).collect(),
        Err(_) => Vec::new(),
    };

    loop {
        println!("Current content:");
        for (i, line) in lines.iter().enumerate() {
            println!("{}: {}", i + 1, line);
        }

        println!("Commands:");
        println!("  - a: Append a new line");
        println!("  - d <line_number>: Delete the specified line");
        println!("  - s: Save and exit");
        println!("  - q: Quit without saving");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap_or("");
        match command {
            "a" => {
                println!("Enter the new line:");
                let mut new_line = String::new();
                io::stdin().read_line(&mut new_line).expect("Failed to read line");
                lines.push(new_line.trim().to_string());
            }
            "d" => {
                let line_number: usize = match parts.next() {
                    Some(s) => match s.parse() {
                        Ok(n) => n,
                        Err(_) => {
                            println!("Invalid line number");
                            continue;
                        }
                    },
                    None => {
                        println!("Missing line number");
                        continue;
                    }
                };
                if line_number <= lines.len() {
                    lines.remove(line_number - 1);
                } else {
                    println!("Invalid line number");
                }
            }
            "s" => {
                match fs::write(filename, lines.join("\n")) {
                    Ok(_) => {
                        println!("File saved successfully");
                        return;
                    }
                    Err(_) => {
                        println!("Failed to save file");
                        return;
                    }
                }
            }
            "q" => {
                println!("Exiting without saving");
                return;
            }
            _ => {
                println!("Unknown command");
            }
        }
    }
}
/// Program starts here