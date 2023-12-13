use std::fs;

fn main() {
    const PATH: &str = "./src/test_input_day5_p1.txt";
    let input: Vec<(String, Vec<Vec<String>>)> = fs::read_to_string(PATH).
        expect("Failed to open file.")
        .split("\n\n")
        // .into_iter()
        .map(|block| block.split_once(":").unwrap())
        .map(|(label, values)| (label.trim().to_owned(), values.trim().lines().map(|line| line.split_whitespace().map(|num| num.to_owned()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>()))
        .collect();

    for (label, map) in input.iter(){
        println!("{} {:?}", label, map);
    }
}
