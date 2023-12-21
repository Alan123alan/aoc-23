use std::fs;

fn main() {
    // let path = "./src/test_day8_p1.txt";
    // let path = "./src/test2_day8_p1.txt";
    let path = "./src/day8_p1.txt";
    let input: Vec<String> = fs::read_to_string(path).expect("Failed to open file").lines().map(|line| line.to_string()).collect();
    let instructions: String = input.first().unwrap().to_string();
    // let map: &Vec<(&str, &str)> = &input[2..].iter().map(|node| node.split_once("=").unwrap()).collect();
    let mut map: Vec<(String, (String, String))> = Vec::new();
    for node in &input[2..]{
        let (id, left_and_right) = node.split_once("=").unwrap();
        let id = id.trim().to_string();
        let left_and_right = left_and_right.split_once(",").unwrap();
        let mut left = left_and_right.0.trim().to_string();
        let mut right = left_and_right.1.trim().to_string();
        left.retain(|ch| ch != '(' && ch != ')');
        right.retain(|ch| ch != '(' && ch != ')');
        // println!("{}", id);
        // println!("{:?}", left_and_right);
        // println!("{} {}", left, right);
        map.push((id, (left, right)));
    }
    let mut looping_condition = true;
    let mut steps = 0;
    let mut index = map.iter().position(|(id, (left, right))| *id == "AAA").unwrap();;
    while looping_condition{
        for instruction in instructions.chars(){
            println!("instruction {}", instruction);
            println!("index: {}", index);
            println!("current map id {}", map[index].0);
            match instruction{
                'L'=>{
                    index = map.iter().position(|(id, (left, right))| *id == map[index].1.0).unwrap();
                },
                'R'=>{
                    index = map.iter().position(|(id, (left, right))| *id == map[index].1.1).unwrap();
                },
                _=>()
            }
            steps += 1;
            if map[index].0 == "ZZZ"{
                looping_condition = false;
                break;
            }
        }
    }
    // while true {
    //     match instruction.chars().into_iter().next(){
    //         'L'=>{
    //             map.iter().position(||)   map[index].1.0
    //         },
    //         'R'=>
    //     }
        
    // }
    // println!("{}", instructions);
    // println!("{:?}", map);
    println!("{}", steps);
    // println!("{:?}", input);
}
