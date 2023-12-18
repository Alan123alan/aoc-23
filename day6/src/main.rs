use std::fs;

fn main() {    
    let path = "./src/test_day6_p1.txt";
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
    for time_limit in time_limits{
        // let time_button_pressed = 0;
        let mut distance_travelled = 0;
        for milliseconds_button_pressed in 0..(time_limit + 1){
            // time_button_pressed += 1;
            distance_travelled = (time_limit - milliseconds_button_pressed) * milliseconds_button_pressed;
            

        }
    }
    // println!("{:?}", time_limits);
    // println!("{:?}", distances_to_beat);

    // println!("{:?}",input);
    // println!("{:?}",time_and_distance);
}
