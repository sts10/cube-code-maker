use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn make_map_for_starting_with_given_letter(
    first_letter: String,
    names: &[String],
) -> HashMap<String, usize> {
    let mut second_and_third_letter_map: HashMap<String, usize> = HashMap::new();
    for name in names {
        let name = clean_name(name.to_string());
        if name.starts_with(&first_letter) && name.len() > 2 {
            let second_letter = name.chars().nth(1).unwrap();
            let third_letter = name.chars().nth(2).unwrap();
            let second_plus_third_letter = format!("{second_letter}{third_letter}");
            second_and_third_letter_map
                .entry(second_plus_third_letter)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }
    second_and_third_letter_map
}

pub fn make_map_for_fourth_and_fifth(names: &[String]) -> HashMap<String, usize> {
    let mut fourth_and_fifth_map: HashMap<String, usize> = HashMap::new();
    for name in names {
        let name = clean_name(name.to_string());
        if name.len() > 4 {
            let fourth_letter = name.chars().nth(3).unwrap();
            let fifth_letter = name.chars().nth(4).unwrap();
            let fourth_plus_fifth_letter = format!("{fourth_letter}{fifth_letter}");
            fourth_and_fifth_map
                .entry(fourth_plus_fifth_letter)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }
    fourth_and_fifth_map
}

pub fn create_cutter_ranges(map: HashMap<String, usize>, start: usize, end: usize) -> Vec<usize> {
    let occurence_total: usize = map.values().sum();
    let mut percentages: Vec<f64> = vec![];

    let alphabet: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();

    for second_letter in alphabet.chars() {
        for third_letter in alphabet.chars() {
            let second_plus_third_letter = format!("{second_letter}{third_letter}");
            match map.get(&second_plus_third_letter) {
                Some(letter_pair_occurences) => percentages
                    .push((*letter_pair_occurences as f64 / occurence_total as f64) as f64),
                None => percentages.push(0.0 as f64),
            };
        }
    }

    let percentage_bucket_width: f64 = 1.0 / (end - start - 9) as f64; // (99-10)/(26*26) I think?
    let mut range_map: Vec<usize> = [].to_vec();
    let mut running_percentage_total: f64 = 0.0;
    let mut number_to_assign: usize = start; //11 or 21 are good choices?

    for percentage in percentages {
        range_map.push(number_to_assign);
        running_percentage_total += percentage;
        if running_percentage_total > percentage_bucket_width {
            // this is a bit shoddy
            running_percentage_total = running_percentage_total - percentage_bucket_width;
            // running_percentage_total = 0.0;
            if number_to_assign < end {
                number_to_assign += 1;
                // if number ends in a 0, skip that number by adding 1
                if number_to_assign.is_multiple_of(10) {
                    number_to_assign += 1;
                }
            }
        }
    }
    range_map
}

fn _pad_and_to_string(num: usize) -> String {
    if num < 9 {
        format!("0{}", num)
    } else {
        num.to_string()
    }
}

fn clean_name(name: String) -> String {
    let mut clean_name = "".to_string();
    let name = name.to_uppercase();
    for c in name.chars() {
        if is_latin_alphabetic(c) {
            clean_name.push(c);
        }
    }
    clean_name.to_string()
}

pub fn is_latin_alphabetic(chr: char) -> bool {
    let chr = chr as u16;
    (chr >= 65 && chr <= 98) || (chr >= 97 && chr <= 122)
}

/// Reads a text file into a Vector of `char`s (characters)
pub fn read_string_from_file_to_vector(file_path: PathBuf) -> io::Result<Vec<char>> {
    let mut f = File::open(file_path)?;
    let mut string_from_file = String::new();
    f.read_to_string(&mut string_from_file)
        .expect("something went wrong reading the file");

    let mut vector_of_chars = Vec::new();
    for c in string_from_file.chars() {
        vector_of_chars.push(c);
    }
    Ok(vector_of_chars)
}
