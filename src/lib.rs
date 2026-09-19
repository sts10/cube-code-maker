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
                .entry(second_plus_third_letter.clone())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }
    if first_letter.to_lowercase() == "b" {
        eprintln!("B map is: {:?}\n", second_and_third_letter_map);
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

    // We don't want our cutter number to end in a 0, so
    // we'll remove 7 numbers for ending in 0 (30, 40, 50, 60, 70, 80, 90)
    let number_of_buckets_we_have_total = (end - start) - 7;
    // This variable occurences_per_bucket_threshold is the number of second+third letter
    // occurance that we want to put in each "bucket", e.g. 21, 22, 23... 99.
    let occurences_per_bucket_threshold: usize = occurence_total / number_of_buckets_we_have_total; // would floor this if I could?

    // We don't want our cutter number to end in a 0, so
    // we'll remove 7 numbers for ending in 0 (30, 40, 50, 60, 70, 80, 90)
    // let percentage_bucket_width: f64 = 1.0 / (end - start - 7) as f64;
    let mut range_map: Vec<usize> = [].to_vec();
    let mut number_of_the_bucket_we_are_assigning_currently: usize = start;
    let mut this_buckets_current_total: usize = 0;

    let alphabet: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();

    for second_letter in alphabet.chars() {
        for third_letter in alphabet.chars() {
            let second_plus_third_letter = format!("{second_letter}{third_letter}");
            let this_pairs_number_of_occurences = match map.get(&second_plus_third_letter) {
                Some(letter_pair_occurences) => letter_pair_occurences,
                None => &0, // never occurred. we'll use 0
            };
            // What would this bucket look like if we added only have of this pair's occurences?
            let this_bucket_total_after_half_this_addition = this_buckets_current_total
                + (*this_pairs_number_of_occurences as f64 / 2.0).ceil() as usize;

            // if it doesn't overflow our current bucket...
            if this_bucket_total_after_half_this_addition <= occurences_per_bucket_threshold {
                // we'll add this pair to this bucket
                this_buckets_current_total =
                    this_buckets_current_total + this_pairs_number_of_occurences;
                range_map.push(number_of_the_bucket_we_are_assigning_currently);
            } else {
                // But! if adding even just half of this pair's occurrences to this bucket
                // would cause it to overflow, let's start a new bucket!
                number_of_the_bucket_we_are_assigning_currently =
                    number_of_the_bucket_we_are_assigning_currently + 1;
                // if our new bucket would end in a zero, add 1 more in order to skip it
                // (see comment above)
                if number_of_the_bucket_we_are_assigning_currently.is_multiple_of(10) {
                    number_of_the_bucket_we_are_assigning_currently =
                        number_of_the_bucket_we_are_assigning_currently + 1;
                }

                this_buckets_current_total = 0;
                range_map.push(number_of_the_bucket_we_are_assigning_currently);
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
