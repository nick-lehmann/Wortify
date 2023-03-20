mod word;
use word::Word;
mod input;
pub use input::Input;
mod dataset;
pub use dataset::{get_word_dataset, Dataset};

static REQUIRED_LETTERS_COUNT: usize = 7;

pub fn solve<'a>(input: &Input, dataset: &'a Dataset) -> Vec<&'a Word> {
    dataset
        .iter()
        .filter_map(|word| {
            if is_solution(input, word) {
                Some(word)
            } else {
                None
            }
        })
        .collect()
}

/// Determines if the given word is a solution for the given input.
///
/// The following conditions must be met to consider the word a solution:
/// - `input.required` must be contained in the word.
/// - Every char in `word` must be contained in `input.letters` or be `input.required`.
fn is_solution(input: &Input, word: &Word) -> bool {
    let mut has_required_letter = false;

    for letter in word.0.chars() {
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
    use crate::{word::Word, Input};

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
}
