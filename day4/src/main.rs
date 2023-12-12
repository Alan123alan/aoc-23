use std::fs;

fn extract_numbers_you_have(card: String)->Vec<String>{
    return card
    .split("|")
    .collect::<Vec<&str>>()[1]
    .trim()
    .split(" ")
    .collect::<Vec<&str>>()
    .iter()
    .filter(|num| **num != "")
    .collect::<Vec<&&str>>()
    .iter()
    .map(|num| String::from(**num))
    .collect();
}

fn extract_winning_numbers(card: String)->Vec<String>{
    return card
    .split("|")
    .collect::<Vec<&str>>()[0]
    .trim()
    .split(" ")
    .collect::<Vec<&str>>()
    .iter()
    .filter(|num| **num != "")
    .collect::<Vec<&&str>>()
    .iter()
    .map(|num| String::from(**num))
    .collect();
}

fn main() {
    //defining path of file to read
    let path = "./src/input_day4_p1.txt";
    //reading file line by line into a vector of Strings
    let input: Vec<String> = fs::read_to_string(path)
        .expect("Failed to read file")
        .lines()
        .map(String::from)
        .collect();
    //splitting input lines into identifier and contents
    // let original_cards: Vec<(&str, &str)> = input.iter().map(|line|(&line[..8],&line[8..])).collect();
    let original_cards: Vec<(&str, &str)> = input.iter().map(|line| line.split_once(": ").unwrap()).collect();
    // let mut cards_counts: Vec<i32> = [1, 1, 1, 1, 1, 1].to_vec();
    let mut cards_counts: Vec<i32> = vec![1;original_cards.len()];
    // let mut all_cards: Vec<(&str, &str)> = original_cards.clone();
    //enumerating with an index all cards and iterating through them
    //is recursion the solution?
    for (index, (card_id, card_contents)) in original_cards.iter().enumerate(){
        //printing card id and card contents
        println!("Index: {}", index);
        println!("'{}''{}'", card_id, card_contents);
        let winning_numbers = extract_winning_numbers(card_contents.to_string());
        let numbers_you_have = extract_numbers_you_have(card_contents.to_string());
        let mut copies = 0;
        for num in winning_numbers.iter(){
            if numbers_you_have.contains(&num){
                copies += 1;
            };
        } 
        println!("Copies won: {}",copies);
        let card_count = cards_counts[index];
        for _ in 0..card_count{
            for i in (index + 1)..(index + 1 + copies){
                cards_counts[i] += 1;
            }
        }
        // if index + copies + 1 < all_cards.len(){
        //     let card_copies = &original_cards.clone()[index+1..index+copies+1];
        // }
    }
    println!("{}", cards_counts.iter().sum::<i32>());
    //     let mut new_cards = cards.clone();
    //     for card_copy in card_copies{
    //         for (index, original) in new_cards.iter().rev().enumerate(){
    //             let (original_id, original_contents) = original;
    //             let (copy_id, copy_contents) = card_copy;
    //             if original_id == copy_id{
    //                 new_cards.insert(index-1, (copy_id, copy_contents));
    //                 println!("{:?}", new_cards);
    //             }
                
    
    //         }
    //     }
    //     println!("{:?}",card_copies);
    // }
}
