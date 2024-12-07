use std::collections::HashMap;
use std::fs::File;
use std::io::{self,  Read, Write};
use clap::Parser;

/// Simple program to encode words from a file
#[derive(Parser)]
struct Args {
    /// Enable small encoding mode
    #[arg(short, long)]
    small: bool,
}

fn main() -> io::Result<()> {
    // Parse command-line arguments
    let args = Args::parse();

    // Define the character to number mapping
    let l2n: HashMap<char, i32> = [
        ('ا', 1), ('آ', 1), ('ب', 2), ('ج', 3), ('د', 4), ('ه', 5), ('و', 6),
        ('ز', 7), ('ح', 8), ('ط', 9), ('ی', 10), ('ک', 20), ('ل', 30), ('م', 40),
        ('ن', 50), ('س', 60), ('ع', 70), ('ف', 80), ('ص', 90), ('ق', 100), ('ر', 200),
        ('ش', 300), ('ت', 400), ('ث', 500), ('خ', 600), ('ذ', 700), ('ض', 800),
        ('ظ', 900), ('غ', 1000),
    ]
    .iter()
    .cloned()
    .collect();

    // Read file content
    let file_content = read_file("content.txt").expect("'content.txt' file does not exsit, try to make one and put your text in it");
    let words: Vec<&str> = file_content.split_whitespace().collect();

    // Encode each word
    let coding: Vec<i32> = words
        .iter()
        .map(|&word| encode(word, &l2n, args.small))
        .collect();

    // Print results
    println!("{:?}", words);
    println!("{:?}", coding);

    // Prompt the user to press Enter to exit
    wait_for_exit();

    Ok(())
}

fn read_file(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn encode(word: &str, l2n: &HashMap<char, i32>, small: bool) -> i32 {
    let mut counter = 0;

    for w in word.chars() {
        if small && w == 'س' {
            counter += 0;
        } else if let Some(&n) = l2n.get(&w) {
            counter += n;
        }
    }

    counter
}

fn wait_for_exit() {
    println!("Press Enter to exit...");
    let _ = io::stdout().flush(); // Ensure the message is printed immediately
    let mut buffer = String::new();
    let _ = io::stdin().read_line(&mut buffer); // Wait for the user to press Enter
}
