use clap::Parser;
use cube_code::create_cutter_ranges;
use cube_code::make_map_for_fourth_and_fifth;
use cube_code::make_map_for_starting_with_given_letter;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Text file with list of authors to map
    #[clap(name = "AUTHORS FILE")]
    authors_file: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let creator_names = read_creators(cli.authors_file);

    let alphabet: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();

    for letter in alphabet.chars() {
        let second_and_third_letter_map =
            make_map_for_starting_with_given_letter(letter.to_string(), &creator_names);

        println!(
            "{}: {:?}",
            letter,
            create_cutter_ranges(second_and_third_letter_map.clone(), 1, 99)
        );
    }
    println!("Additional (fourth and on) letter map:");
    let fourth_and_fifth_map = make_map_for_fourth_and_fifth(&creator_names);

    println!(
        "Additional: {:?}",
        create_cutter_ranges(fourth_and_fifth_map.clone(), 21, 99)
    );
}

fn read_creators(file_name: PathBuf) -> Vec<String> {
    let mut creators = vec![];
    let f = match File::open(file_name) {
        Ok(file) => file,
        Err(_e) => panic!("Error opening creators-names txt file"),
    };
    let file = BufReader::new(&f);
    for line in file.lines() {
        match line {
            Ok(line) => creators.push(line),
            Err(_e) => continue,
        };
    }
    creators
}
fn _write_json_out(map: HashMap<char, usize>) -> std::io::Result<()> {
    let json_string = serde_json::to_string(&map).expect("Failed to serialize HashMap to JSON");
    let mut file = File::create("map.json")?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}
