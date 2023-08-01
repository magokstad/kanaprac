use std::collections::HashMap;
use std::io::{self, Write};

use clap::{Parser, ValueEnum};
use colored::Colorize;
use rand::seq::{IteratorRandom, SliceRandom};


// CLI STUFF

#[derive(Parser)]
#[command(name = "KanaPrac")]
#[command(author = "Magnus Økstad")]
#[command(version = "1.0")]
#[command(about = "Practice recognizing the Japanese Kana", long_about = None)]
struct Cli {
    /// Which Kana to use
    #[arg(value_enum, default_value_t = Kana::Hira)]
    kana: Kana,
    /// How many questions you want
    #[arg(short, long)]
    iterations: Option<u8>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Kana {
    /// Only Hiragana
    Hira,
    /// Only Katakana
    Kata,
    /// Both Hiragana and Katakana
    Both
}

// END OF CLI STUFF

fn parse_mapping(data: &str) -> io::Result<HashMap<String, String>> {
    let mut map = HashMap::new(); 

    for line in data.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        if parts.len() == 0 {
            continue
        }
        if parts.len() == 1 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Bad number of arguments on this line: {}", line)));
        }
        for i in 1..parts.len() {
            map.insert(parts[0].to_string(), parts[i].to_string());
        }
    }

    Ok(map)
}

fn iteration(key: &str, val : &str) {

    let mut input = String::new();

    print!("{} ", key);
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.pop();

    if input.eq(val) {
        println!("{}  Good!", "✓".green());
    } else {
        println!("{}  {} was the right answer.", "✘".red(), val.yellow());
    }
}

fn game(kana: Kana, iterations: Option<u8>) {
    let hira_data = include_str!("../data/hiragana.txt");
    let kata_data = include_str!("../data/katakana.txt");
    let hiramap = parse_mapping(hira_data).unwrap();
    let katamap = parse_mapping(kata_data).unwrap();
    let map = match kana {
        Kana::Hira => hiramap,
        Kana::Kata => katamap,
        Kana::Both => hiramap.into_iter().chain(katamap.into_iter()).collect()
    };

    let keys: Vec<&String> = map.keys().collect();

    match iterations {
        Some(iter) => {
            for _ in 0..iter {
                let (key, val) = map.iter().choose(&mut rand::thread_rng()).unwrap();
                iteration(key, val);
            }
        },
        None => {
            loop {
                let (key, val) = map.iter().choose(&mut rand::thread_rng()).unwrap();
                iteration(key.as_str(), val);
            }
        }
    }
}

// TODO: Scoring!
// TODO: Remove from draw-pool on success
//          - When draw-pool empty, reinit
// TODO: Add rest of hiragana
// TODO: Add katakana

fn main() {
    let cli = Cli::parse();
    game(cli.kana, cli.iterations)
}
