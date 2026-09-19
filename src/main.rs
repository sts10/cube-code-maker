use clap::Parser;
use cube_code::create_cutter_ranges;
use cube_code::make_map_for_fourth_and_fifth;
use cube_code::make_map_for_starting_with_given_letter;
// use std::collections::HashMap;
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
    let creator_names_file = cli.authors_file;
    let creator_names = read_creators(creator_names_file.clone());
    // Could make these CLI parameters...
    let first_cutter_number_to_assign = 22;
    let last_cutter_number_to_assign = 99;

    let alphabet: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();

    for letter in alphabet.chars() {
        let second_and_third_letter_map =
            make_map_for_starting_with_given_letter(letter.to_string(), &creator_names);

        let mut bucket_overflow_sensitivity = 1.5;
        let mut bucket_overflow_step = 0.05;
        let mut this_letters_cutter_ranges = create_cutter_ranges(
            second_and_third_letter_map.clone(),
            first_cutter_number_to_assign,
            last_cutter_number_to_assign,
            bucket_overflow_sensitivity,
        );
        let mut last_cutter_number_assigned =
            this_letters_cutter_ranges[this_letters_cutter_ranges.len() - 1];

        while last_cutter_number_assigned < 94 || last_cutter_number_assigned > 103 {
            bucket_overflow_sensitivity = bucket_overflow_sensitivity + bucket_overflow_step;
            this_letters_cutter_ranges = create_cutter_ranges(
                second_and_third_letter_map.clone(),
                first_cutter_number_to_assign,
                last_cutter_number_to_assign,
                bucket_overflow_sensitivity,
            );
            last_cutter_number_assigned =
                this_letters_cutter_ranges[this_letters_cutter_ranges.len() - 1];
            if last_cutter_number_assigned > 97 && letter != 'X' {
                eprintln!("I overshot to {}", last_cutter_number_assigned);
                // Cutter numbers went too high!
                bucket_overflow_sensitivity = 1.4;
                bucket_overflow_step = bucket_overflow_step / 2.0;
            }
        }

        eprintln!(
            "For letter {}, last cutter assigned was {}",
            letter, last_cutter_number_assigned
        );

        append_ranges_out(
            Some(letter.to_string()),
            this_letters_cutter_ranges,
            creator_names_file.clone(),
        )
        .unwrap();
    }

    let fourth_and_fifth_map = make_map_for_fourth_and_fifth(&creator_names);
    append_ranges_out(
        None,
        create_cutter_ranges(
            fourth_and_fifth_map.clone(),
            first_cutter_number_to_assign,
            last_cutter_number_to_assign,
            3.6,
        ),
        creator_names_file.clone(),
    )
    .unwrap();
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
use std::fs::OpenOptions;
fn append_ranges_out(
    letter: Option<String>,
    ranges: Vec<usize>,
    inputted_file_name: PathBuf,
) -> std::io::Result<()> {
    let output_file_name = format!("{}-cube-map.txt", inputted_file_name.display());
    // let mut file = File::create(output_file_name)?;
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(output_file_name)?;
    match letter {
        Some(letter) => writeln!(file, "{}: [", letter)?,
        None => writeln!(file, "additionalCharacters: [")?,
    }

    for num in ranges {
        write!(file, "{}, ", num)?;
    }

    write!(file, "];\n\n")?;
    Ok(())
}
