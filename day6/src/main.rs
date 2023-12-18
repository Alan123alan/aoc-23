use std::fs;

fn day6_p1(path:&str){
    let input: Vec<String> = fs::read_to_string(path).expect("Failed to read file.").lines().map(|line| line.to_string()).collect();
    let time_and_distance: Vec<(&str, &str)>= input.iter().map(|line| line.split_once(":").unwrap()).collect();
    let mut time_limits: Vec<i32> = Vec::new();
    let mut distances_to_beat: Vec<i32> = Vec::new();
    for (label, values) in time_and_distance.iter(){
        match *label {
            "Time" => {
                for value in values.trim().split_whitespace().collect::<Vec<&str>>(){
                    time_limits.push(value.parse().unwrap());
                }
            },
            "Distance" => {
                for value in values.trim().split_whitespace().collect::<Vec<&str>>(){
                    distances_to_beat.push(value.parse().unwrap());
                }
            },
            _=>()
            
        }
    }
    let mut results: Vec<Vec<i32>> = Vec::new();
    for time_limit in time_limits{
        let mut result: Vec<i32> = Vec::new();
        for milliseconds_button_pressed in 0..(time_limit + 1){
            let distance_travelled = (time_limit - milliseconds_button_pressed) * milliseconds_button_pressed;
            // println!("{:?}", distance_travelled);
            result.push(distance_travelled);
        }
        results.push(result);
    }
    println!("{:?}", results);
    let mut counts: Vec<i32> = Vec::new();
    for (index, distance_to_beat)in distances_to_beat.iter().enumerate(){
        let mut count = 0;
        for distance_travelled in results[index].iter(){
            if distance_travelled > distance_to_beat{
                count += 1;
            }
        }
        counts.push(count);
    }
    println!("{:?}", counts.iter().fold(1, |acc, el| acc * el));
}

fn day6_p2(path:&str){
    let input: Vec<String> = fs::read_to_string(path).expect("Failed to read file.").lines().map(|line| line.to_string()).collect();
    let times_and_distances: Vec<(&str, &str)>= input.iter().map(|line| line.split_once(":").unwrap()).collect();
    let mut time_limit: i64 = 0;
    let mut distance_to_beat: i64= 0;
    for (label, values) in times_and_distances.iter(){
        let mut value = values.to_string();
        value.retain(|ch| !ch.is_whitespace());
        match *label {
            "Time" => time_limit = value.parse().unwrap(),
            "Distance" => distance_to_beat = value.parse().unwrap(),
            _ => ()
        }
        println!("{} : {}", label, value);
    }
    println!("{} : {}", time_limit, distance_to_beat);
    // let mut results: Vec<Vec<i32>> = Vec::new();
    let mut results: Vec<i64> = Vec::new();
    for milliseconds_button_pressed in 0..(time_limit + 1){
        let distance_travelled = (time_limit - milliseconds_button_pressed) * milliseconds_button_pressed;
        // println!("{:?}", distance_travelled);
        if distance_travelled > distance_to_beat{
            results.push(distance_travelled);
        }
    }
        // results.push(result);
    println!("{:?}", results.len());
    // let mut counts: Vec<i32> = Vec::new();
    // for (index, distance_to_beat)in distances_to_beat.iter().enumerate(){
    //     let mut count = 0;
    //     for distance_travelled in results[index].iter(){
    //         if distance_travelled > distance_to_beat{
    //             count += 1;
    //         }
    //     }
    //     counts.push(count);
    // }
    // println!("{:?}", counts.iter().fold(1, |acc, el| acc * el));

}

fn main() {    
    let path = "./src/day6_p1.txt";
    // day6_p1(path);
    day6_p2(path);
}
