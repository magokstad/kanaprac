use clap::{Parser, ValueEnum};

mod mapping;
mod io;
use mapping::Mapping;
use io::{get_user_translation, print_correct, print_wrong};


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
    /// How many full loops
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


fn iteration(mapping: &mut Mapping) {

    let kana = mapping.get_random();

    let input = get_user_translation(&kana);

    let mut correct = false;
    for roma in mapping.get_romaji_from(&kana) {
        if roma.eq(&input) {
            correct = true;
        }
    } 
    if correct {
       print_correct(mapping, &kana) 
    } else {
        print_wrong(mapping, &kana)
    }
}

fn game(kana: Kana, iterations: Option<u8>) {
    let hira_map = Mapping::from(include_str!("../data/hiragana.txt")).unwrap();
    let kata_map = Mapping::from(include_str!("../data/katakana.txt")).unwrap();

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
