use std::fs;

fn main() {
    let path = "./src/input_day4_p1.txt";
    let input: Vec<String> = fs::read_to_string(path)
        .expect("Failed to read file")
        .lines()
        .map(String::from)
        .collect();
    let mut card_total = 0;
    const BASE:i32 = 2;
    for line in input.clone(){
        let card = &line[8..];
        let winning_numbers:Vec<&str> = card
            .split("|")
            .collect::<Vec<&str>>()[0]
            .trim()
            .split(" ")
            .collect::<Vec<&str>>(); 
        let numbers_you_have: Vec<&str> = card
            .split("|")
            .collect::<Vec<&str>>()[1]
            .trim()
            .split(" ")
            .collect::<Vec<&str>>();
        let winning_numbers: Vec<&&str> = winning_numbers.iter().filter(|num| **num != "").collect();
        let numbers_you_have: Vec<&&str> = numbers_you_have.iter().filter(|num| **num != "").collect();
        let mut power = 0;
        for num in winning_numbers.clone(){
            if numbers_you_have.contains(&num){
                println!("Numbers you have {:?}",numbers_you_have);
                println!("Winning number {}",num);
                power += 1;
            };
        }
        if power != 0 {
            card_total += BASE.pow(power-1);
        }
        // println!("{}",card);
        println!("{}",card_total);
        // println!("{:?}",winning_numbers);
    }
    // println!("{:?}",input);
}
