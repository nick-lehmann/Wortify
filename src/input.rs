use std::collections::HashSet;

use crate::REQUIRED_LETTERS_COUNT;

pub struct Input {
    pub required: char,
    pub letters: [char; 6],
}

impl TryFrom<&str> for Input {
    type Error = &'static str;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let parts: Vec<char> = line.trim().chars().collect();
        println!("Parts {:?}", parts);

        if parts.len() != REQUIRED_LETTERS_COUNT {
            return Err("Invalid input. Please enter 7 letters.");
        }

        let letter_set: HashSet<char> = HashSet::from_iter(parts.clone());
        if letter_set.len() != REQUIRED_LETTERS_COUNT {
            return Err("Invalid input. Please enter 7 unique letters.");
        }

        let required: char = parts[0];
        let letters = [parts[1], parts[2], parts[3], parts[4], parts[5], parts[6]];
        Ok(Self { required, letters })
    }
}
