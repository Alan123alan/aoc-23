use std::fs;
// use std::collections::HashMap;

fn read_input(path:&str){
    let input = fs::read_to_string(path).expect("Should have worked");
    let lines: Vec<&str> = input.split("\n").collect();
    //an iterator is lazy, you need to consume it with collect
    let game_records: Vec<Vec<&str>> = lines.iter().map(|line| line.split(":").collect()).collect();
    let game_ids: Vec<i32> = game_records.iter().map(|game_record| game_record[0].split_once(" ").unwrap().1.parse().unwrap()).collect();
    let cube_subsets: Vec<&str> = game_records.iter().map(
        |game_record| game_record[1]
    ).collect();
    let cube_subsets: Vec<Vec<&str>> = cube_subsets.iter().map(
        |cube_subset| cube_subset.split(";").collect()
    ).collect();
    let cube_subsets: Vec<Vec<Vec<&str>>> = cube_subsets.iter().map(
        |cube_subset| cube_subset.iter().map(
            |colors|colors.split(",").collect()
        ).collect()
    ).collect();
    let cube_subsets: Vec<Vec<Vec<(&str, &str)>>> = cube_subsets.iter().map(
        |cube_subset| cube_subset.iter().map(
            |colors| colors.iter().map(
                |color| color.trim().split_once(" ").unwrap()
            ).collect()
        ).collect()
    ).collect();
    // let mut possible_games_sum = 0;
    let mut set_power = 0;
    let mut power_sum = 0;
    for index in 0..game_ids.len(){
        println!("game_id:{}",game_ids[index]);
        // let mut is_game_possible = true;
        let mut red = Vec::<i32>::new();
        let mut green = Vec::<i32>::new();
        let mut blue = Vec::<i32>::new();
        for colors in &cube_subsets[index]{
            for color in colors{
                let number_of_cubes:i32= color.0.parse().unwrap();
                let cube_color = color.1;
                // println!("{number_of_cubes}:{cube_color}");
                match cube_color {
                    "red"=> red.push(number_of_cubes),
                    "green"=> green.push(number_of_cubes),
                    "blue"=> blue.push(number_of_cubes),
                    _ => ()
                };
                // if !is_game_possible{
                //     break;
                // }
            }
            // if !is_game_possible{
            //     break;
            // }
        }
        let min_red = red.iter().max().unwrap();
        let min_green = green.iter().max().unwrap();
        let min_blue = blue.iter().max().unwrap();
        set_power = min_red * min_green * min_blue;
        power_sum += set_power;
        // println!("red:{red}");
        // println!("green:{green}");
        // println!("blue:{blue}");
        // if is_game_possible{
        //     println!("{}",game_ids[index]);
        //     possible_games_sum += game_ids[index];
        // }
        println!("{set_power}");
    }
    println!("{power_sum}");
    // for cube_subset in cube_subsets{
    //     for colors in cube_subset{
    //         for color in colors{
    //             let number_of_cubes = color.0;
    //             let cube_color = color.1;
    //             println!("{number_of_cubes}:{cube_color}");
    //         }
    //     }
    // }
    // println!("{:?}", game_records);
    // for game_record in game_records.iter(){
    //     let game_id = game_record[0];
    //     let cubes_subset = game_record[1];
    // }
    // println!("{:?}", game_ids);
    // println!("{:?}", cube_subsets);
    // let mut game_hashmap:HashMap<&str, &HashMap<&str,&str>> = HashMap::new();
    // for line in lines.iter(){
    //     let game_record: Vec<&str> = line.split(":").collect();
    //     let game_id= game_record[0];
    //     println!("{}",game_id);
    //     let cubes_subset: Vec<&str> = game_record[1].split(";").collect();
    //     let mut color_hashmap:HashMap<&str,&str> = HashMap::new();
    //     for subset in cubes_subset.iter(){ 
    //         let colors: Vec<&str> = subset.split(",").collect();
    //         for color in colors.iter(){
    //             let key_val: Vec<&str> = color.trim().split(" ").collect();
    //             color_hashmap.insert(key_val[1], key_val[0]);
    //             println!("{:?}",key_val);
    //         }
    //         game_hashmap.insert(game_id, &color_hashmap);
    //         println!("{subset}");
    //         println!("{:?}",colors);
    //     }
    //     // println!("{:?}", line);
    //     // println!("{:?}", game_record);
    //     // println!("{}", game_id);
    //     // println!("{:?}", cubes_subset);
    // }
    // println!("{:?}",lines);
}

fn main() {
    read_input("./src/input_day2_p1.txt")
}
