use std::io::{self, Write};

use clap::{Parser, ValueEnum};
use colored::Colorize;

mod mapping;
use mapping::Mapping;


// CLI STUFF
#[derive(Parser)]
#[command(name = "KanaPrac")]
#[command(author = "Magnus Økstad")]
#[command(version = "0.2")]
#[command(about = "Practice recognizing the Japanese Kana", long_about = None)]
struct Cli {
    /// Which Kana to use
    #[arg(value_enum, default_value_t = Kana::Hira)]
    kana: Kana,
    /// How many loops do you want
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


fn gen_answer_string(mapping: &Mapping, kana: &String) -> String {
    let romaji = mapping.get_romaji_from(kana);
    let colored: Vec<String> = romaji.iter().map(|x| {x.yellow().to_string()}).collect();
    colored.join(" or ")
}

fn gen_score_string(mapping: &Mapping) -> String{
    let (hi, lo) = mapping.work_set_status();
    return format!("({}/{})", lo, hi);
}

fn iteration(mapping: &mut Mapping) {

    let mut input = String::new();
    let kana = mapping.get_random();

    print!("{} ", kana);
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.pop();

    let mut correct = false;
    for roma in mapping.get_romaji_from(&kana) {
        if roma.eq(&input) {
            correct = true;
        }
    } 
    if correct {
        let has_more = mapping.remove(&kana);
        println!("{}  Good! {}\n", "✓".green(), gen_score_string(&mapping));
        if !has_more {
            println!("{}", ">< >< FULL LOOP >< ><\n".blue())
        }
    } else {
        println!("{}  {} was the right answer.\n", "✘".red(), gen_answer_string(&mapping, &kana));
    }
}

fn game(kana: Kana, iterations: Option<u8>) {
    let hira_data = include_str!("../data/hiragana.txt");
    let kata_data = include_str!("../data/katakana.txt");
    let hira_map = Mapping::from(hira_data).unwrap();
    let kata_map = Mapping::from(kata_data).unwrap();
    let mut map = match kana {
        Kana::Hira => hira_map,
        Kana::Kata => kata_map,
        Kana::Both => hira_map.join(&kata_map)
    };

    match iterations {
        Some(iter) => {
            for _ in 0..iter {
                iteration(&mut map);
            }
        },
        None => {
            loop {
                iteration(&mut map);
            }
        }
    }
}

// TODO: Scoring!
// TODO: Remove from draw-pool on success
//          - When draw-pool empty, reinit
// TODO: Add rest of hiragana and katakana

fn main() {
    let cli = Cli::parse();
    game(cli.kana, cli.iterations)
}
