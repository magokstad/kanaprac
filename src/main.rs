use std::collections::{HashMap, HashSet};
use std::io::{self, Write};

use clap::{Parser, ValueEnum};
use colored::Colorize;
use rand::seq::IteratorRandom;



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



struct Mapping {
    map: HashMap<String, Vec<String>>,
    kana: HashSet<String>,
    work_set: HashSet<String>
}

impl Mapping {
    pub fn new(map: &HashMap<String, Vec<String>>, kana: &HashSet<String>) -> Self {
        return Mapping { map: map.clone(), kana: kana.clone(), work_set: kana.clone() }
    }

    pub fn join(&self, other: &Mapping) -> Mapping {
        let map = self.map
            .clone()
            .into_iter()
            .chain(other.map.clone().into_iter())
            .collect();
        let kana = self.kana
            .clone()
            .into_iter()
            .chain(other.kana.clone().into_iter())
            .collect();
        return Mapping::new(&map, &kana);
    }

    pub fn get_random(&mut self) -> String {
        self.ensure_not_empty();
        self.work_set
            .iter()
            .choose(&mut rand::thread_rng())
            .expect("bad").to_owned()
    }

    pub fn remove(&mut self, kana: &String) -> bool {
        self.work_set.remove(kana);
        self.ensure_not_empty()
    }

    fn ensure_not_empty(&mut self) -> bool {
        if self.work_set.is_empty() {
            self.work_set = self.kana.clone();
            return false;
        } 
        return true;
    }
}

fn parse_mapping(data: &str) -> io::Result<Mapping> {
    let mut map : HashMap<String, Vec<String>> = HashMap::new(); 
    let mut kana: HashSet<String> = HashSet::new();

    for line in data.lines() {
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        if parts.len() == 0 {
            continue
        }
        if parts.len() == 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData, 
                format!("Bad number of arguments on this line: {}", line)
            ));
        }
        map.entry(parts[0].to_string()).or_insert(vec![]);
        for i in 1..parts.len() {
            map.get_mut(&parts[0].to_string())
                .unwrap()
                .push(parts[i].to_string());
        }
        kana.insert(parts[0].to_string());
    }

    Ok(Mapping::new(&map, &kana))
}

fn gen_answer_string(mapping: &Mapping, kana: &String) -> String {
    let romaji = mapping.map.get(kana).unwrap().clone();
    let colored: Vec<String> = romaji.iter().map(|x| {x.yellow().to_string()}).collect();
    colored.join(" or ")
}

fn gen_score_string(mapping: &Mapping) -> String{
    let hi = mapping.kana.len();
    let lo = hi - mapping.work_set.len();
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
    for roma in mapping.map.get(&kana).unwrap() {
        if roma.eq(&input) {
            correct = true;
        }
    } 
    if correct {
        let has_more = mapping.remove(&kana);
        println!("{}  Good! {}\n", "✓".green(), gen_score_string(&mapping));
        if !has_more {
            println!("{}", "<> <> FULL LOOP <> <>\n".blue())
        }
    } else {
        println!("{}  {} was the right answer.\n", "✘".red(), gen_answer_string(&mapping, &kana));
    }
}

fn game(kana: Kana, iterations: Option<u8>) {
    let hira_data = include_str!("../data/hiragana.txt");
    let kata_data = include_str!("../data/katakana.txt");
    let hiramap = parse_mapping(hira_data).unwrap();
    let katamap = parse_mapping(kata_data).unwrap();
    let mut map = match kana {
        Kana::Hira => hiramap,
        Kana::Kata => katamap,
        Kana::Both => hiramap.join(&katamap)
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
