use crate::word::Word;
use log::debug;
use std::fs::{self, read_to_string};

static DATASET_URL: &'static str = "https://6mal5.com/wortify/data/words.csv?3";
static APP_NAME: &'static str = "wortify";
static DATASET_FILENAME: &'static str = "words.csv";

pub type Dataset = Vec<Word>;

/// Returns the dataset of all words recognised by Wortify.
///
/// It tries to find a local copy of the dataset first. If it can't find one, it will download it from the server.
pub fn get_word_dataset() -> Dataset {
    let dirs =
        xdg::BaseDirectories::with_prefix(APP_NAME).expect("Unable to get XDG base directories.");

    let dataset_file = dirs.find_data_file(DATASET_FILENAME);

    if let Some(f) = dataset_file {
        debug!("Read dataset from file: {:?}", f);
        let dataset = read_to_string(f).expect("Unable to read dataset file");
        return convert_to_dataset(dataset);
    }

    debug!("Fetch dataset from server: {:?}", DATASET_URL);
    let raw = fetch();

    let path = dirs
        .place_data_file(DATASET_FILENAME)
        .expect("Unable to create file for storing dataset.");

    debug!("Write dataset to file: {:?}", path);
    fs::write(path, raw.clone()).expect("Unable to write dataset to disk");

    convert_to_dataset(raw)
}

/// Converts the raw `.csv` file into a dataset.
fn convert_to_dataset(raw: String) -> Dataset {
    raw.lines().map(|line| Word::from(line)).collect()
}

/// Fetches the dataset as a `.csv` file from the server.
///
/// It is not really a csv as there is only one column.
fn fetch() -> String {
    reqwest::blocking::get(DATASET_URL)
        .expect("Failed to get data from server.")
        .text()
        .unwrap()
}
