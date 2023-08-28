use std::io::{self, Write};

use colored::Colorize;

use crate::mapping::Mapping;

pub fn gen_answer_string(mapping: &Mapping, kana: &str) -> String {
    let romaji = mapping.get_romaji_from(kana);

    let colored: Vec<String> = romaji.iter().map(|x| x.yellow().to_string()).collect();

    colored.join(" or ")
}

pub fn gen_score_string(mapping: &Mapping) -> String {
    let (lo, hi) = mapping.work_set_status();

    format!("({}/{})", lo, hi)
}

pub fn print_correct(mapping: &mut Mapping, kana: &str) {
    let has_more = mapping.remove(kana);
    println!("{}  Good! {}\n", "✓".green(), gen_score_string(mapping));
    if !has_more {
        println!("{}", ">< >< FULL LOOP >< ><\n".blue())
    }
}

pub fn print_wrong(mapping: &Mapping, kana: &str) {
    println!(
        "{}  {} was the right answer.\n",
        "✘".red(),
        gen_answer_string(mapping, kana)
    );
}

pub fn get_user_translation(kana: &String) -> String {
    print!("{} ", kana);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.pop();

    input
}
