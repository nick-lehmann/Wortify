mod word;
use std::collections::HashSet;

use word::Word;
mod input;
pub use input::Input;
mod dataset;
pub use dataset::{get_word_dataset, Dataset};

static REQUIRED_LETTERS_COUNT: usize = 7;

pub struct Solution<'a> {
    pub points: u64,
    pub words: Vec<&'a Word>,
}

pub fn solve<'a>(input: &Input, dataset: &'a Dataset) -> Solution<'a> {
    let words: Vec<&'a Word> = dataset
        .iter()
        .filter_map(|word| {
            if is_solution(input, word) {
                Some(word)
            } else {
                None
            }
        })
        .collect();

    let points: u64 = words.iter().map(|word| points(&word)).sum();

    Solution { points, words }
}

/// Wortify awards you points for each word found. The amount of points depends
/// on a few simple rules that are implemented in this function.
fn points(word: &str) -> u64 {
    let l = word.len() as u64;

    let unique_chars = word.chars().collect::<HashSet<_>>().len() as u64;
    if unique_chars == 7 {
        return l + 7;
    }

    if l <= 4 {
        1
    } else {
        l
    }
}

/// Determines if the given word is a solution for the given input.
///
/// The following conditions must be met to consider the word a solution:
/// - `input.required` must be contained in the word.
/// - Every char in `word` must be contained in `input.letters` or be `input.required`.
fn is_solution(input: &Input, word: &Word) -> bool {
    let mut has_required_letter = false;

    for letter in word.chars() {
        if letter == input.required {
            has_required_letter = true;
            continue;
        }

        if !input.letters.contains(&letter) {
            return false;
        }
    }

    has_required_letter
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, fs::read_to_string};

    use crate::{dataset::convert_to_dataset, get_word_dataset, solve, word::Word, Input};

    use super::is_solution;

    struct SolutionTestcase {
        name: &'static str,
        letters: &'static str,
        word: &'static str,
        solution: bool,
    }

    const SOLUTION_TESTCASES: &[SolutionTestcase] = &[
        SolutionTestcase {
            name: "Solution contains all letter exactly once",
            letters: "abcdefg",
            word: "abcdefg",
            solution: true,
        },
        SolutionTestcase {
            name: "Word contains only required letter",
            letters: "abcdefg",
            word: "a",
            solution: true,
        },
        SolutionTestcase {
            name: "Empty word does not contain required letter",
            letters: "abcdefg",
            word: "",
            solution: false,
        },
        SolutionTestcase {
            name: "Word contains required letter but also others",
            letters: "abcdefg",
            word: "auiop",
            solution: false,
        },
        SolutionTestcase {
            name: "No common letters",
            letters: "abcdefg",
            word: "hijklmnop",
            solution: false,
        },
    ];

    #[test]
    fn test_is_solution() {
        for testcase in SOLUTION_TESTCASES {
            let input = Input::try_from(testcase.letters).unwrap();
            let word = Word::from(testcase.word);
            assert_eq!(
                is_solution(&input, &word),
                testcase.solution,
                "Testcase: {}",
                testcase.name
            );
        }
    }

    #[test]
    fn compare_to_real_world() {
        let raw_solutions =
            read_to_string("/Users/nick/Projekte/wortify/test/2023-03-20.txt").unwrap();

        let input = Input::try_from("eanrgbt").unwrap();
        let dataset = get_word_dataset();

        let expected_points = 4415;
        let expected_words = convert_to_dataset(raw_solutions);

        let solution = solve(&input, &dataset);
        let found_words: HashSet<Word> =
            HashSet::from_iter(solution.words.into_iter().map(|s| s.to_owned()));

        assert_eq!(
            found_words,
            expected_words,
            "False positives: {:?}, false negatives: {:?}",
            found_words.difference(&expected_words),
            expected_words.difference(&found_words)
        );
        assert_eq!(solution.points, expected_points);
    }
}
