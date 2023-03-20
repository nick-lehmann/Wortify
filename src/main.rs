use wortify::{get_word_dataset, solve, Input};

fn main() {
    println!("Please enter your letters & the first one should be the mandatoy middle letter:");

    // Example input from 2023-03-20: eanrgbt
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();

    let input = Input::try_from(line.as_str()).unwrap();
    let dataset = get_word_dataset();

    let solutions = solve(&input, &dataset);
    println!("Found {:?} solutions.", solutions.len());
    for solution in &solutions {
        println!("- {}", solution.0);
    }
}
