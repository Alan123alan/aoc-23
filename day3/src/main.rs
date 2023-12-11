use std::fs;

fn read_file_to_lines(path:&str)-> Vec<String>{
    return fs::read_to_string(path)
    .expect("Failed to read file.")
    .lines()
    .map(String::from)
    .collect();
}


fn check_for_adjacent_symbol(processed_input: Vec<Vec<char>>, row: usize, col: usize)->bool{
    const ULC: (i32,i32) = (-1, -1);
    const U: (i32,i32) = (-1, 0);
    const URC: (i32,i32) = (-1, 1);
    const L: (i32,i32) = (0, -1);
    const R: (i32,i32) = (0, 1);
    const LLC: (i32,i32) = (1, -1);
    const D: (i32,i32) = (1, 0);
    const LRC: (i32,i32) = (1, 1);
    let adjacents: [(i32,i32) ; 8] = [ULC, U, URC, L, R, LLC, D, LRC];
    let rows = processed_input.len();
    let cols = processed_input[0].len();
    for adj in adjacents{
        let new_row = row as i32 + adj.0;
        let new_col = col as i32 + adj.1;
        if new_row >= 0 && new_col >= 0 && new_row < rows as i32 && new_col < cols as i32{
            // let current_element = processed_input[row][col]; 
            let adjacent_element = processed_input[new_row as usize][new_col as usize]; 
            // println!("{} is adjacent to {}",adjacent_element, current_element);
            match adjacent_element{
                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'.' =>(),
                _=>return true,
            }
        }
    }
    return false;
}

//Note: use dfs to get the numbers?
fn is_gear(processed_input: Vec<Vec<char>>, row: usize, col: usize)->bool{
    const ULC: (i32,i32) = (-1, -1);
    const U: (i32,i32) = (-1, 0);
    const URC: (i32,i32) = (-1, 1);
    const L: (i32,i32) = (0, -1);
    const R: (i32,i32) = (0, 1);
    const LLC: (i32,i32) = (1, -1);
    const D: (i32,i32) = (1, 0);
    const LRC: (i32,i32) = (1, 1);
    let adjacents: [(i32,i32) ; 8] = [ULC, U, URC, L, R, LLC, D, LRC];
    let rows = processed_input.len();
    let cols = processed_input[0].len();
    // let mut adjacent_part_count = 0;
    for adj in adjacents{
        let new_row = row as i32 + adj.0;
        let new_col = col as i32 + adj.1;
        if new_row >= 0 && new_col >= 0 && new_row < rows as i32 && new_col < cols as i32{
            let adjacent_element = processed_input[new_row as usize][new_col as usize]; 
            match adjacent_element{
                '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
                    let num = left_right_num_search(processed_input.clone(), new_row as usize, new_col as usize);
                    println!("num: {}", num);
                },
                _=>(),
            }
        }
    }
    return true;
    // return adjacent_part_count == 2;
}

fn left_right_num_search(processed_input: Vec<Vec<char>>, row: usize, col: usize)->String{
    // let rows = processed_input.len();
    let cols = processed_input[0].len();
    let mut left = (col as i32) - 1;
    let mut right = (col as i32) + 1;
    let mut left_num = String::new();
    let mut right_num = String::new();
    let mut num = String::new();
    println!("{}", processed_input[row][left as usize]);
    println!("{}", processed_input[row][right as usize]);
    println!("{}", left as usize);
    println!("{}", right);
    println!("{}", row);
    if left >= 0 && right < cols as i32{
        println!("inside the if");
        while match processed_input[row][left as usize]{
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'=>true,
            _=>false
        }{
            left_num.push(processed_input[row][left as usize]);
            println!("passed the num.push()");
            left -= 1;
            if left < 0 {
                break;
            }
        }
        while match processed_input[row][right as usize]{
            '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'=>true,
            _=>false 
        }{
            right_num.push(processed_input[row][right as usize]);
            right += 1;
            if right >= cols as i32{
                break;
            }
        }
        num.push_str(&left_num);
        num.push(processed_input[row][col]);
        num.push_str(&right_num);
        return num;
    }
    return num;
    //Capture all numbers then leave only unique? what if two numbers are exactly the same?
    //Save their start index too?
}

fn main() {
    //read file into a string
    let input_lines = read_file_to_lines("./src/test_input_day3_p1.txt");
    let processed_input: Vec<Vec<char>> = input_lines.iter().map(|line|line.chars().collect()).collect();
    println!("{:?}",processed_input);
    let rows = processed_input.len();
    let cols = processed_input[0].len();
    // let mut num = String::new();
    // let mut nums: Vec<(String, bool)> = Vec::new();
    // let mut has_adjacent_symbol = false;
    for row in 0..rows{
        for col in 0..cols{
            let current_element = processed_input[row][col];
        //     match current_element {
        //         '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9' => {
        //             if check_for_adjacent_symbol(processed_input.clone(), row, col) && !has_adjacent_symbol{
        //                 has_adjacent_symbol = true;
        //             }
        //             num.push(current_element)
        //         },
        //         _ => {
        //             nums.push((num.clone(), has_adjacent_symbol));
        //             nums.push((current_element.to_string(), false));
        //             has_adjacent_symbol = false;
        //             num.clear();    
        //         },
        //         // _ => () 
        //     }
        // }
        // //special case for when there is no . or other non digit between two differnt numbers that span two rows
        // nums.push((num.clone(), has_adjacent_symbol));
        // has_adjacent_symbol = false;
        // num.clear();
        match current_element{
            '*'=>{
                is_gear(processed_input.clone(), row, col);
            },
            _=>()
        }
    }
}

    // println!("rows {}", rows);
    // println!("cols {}", cols);
    // println!("{:?}", nums);
    // println!("nums rows: {}", nums.len());
    // let mut sum: i32 = 0;
    // for (num, add) in nums{
    //     if add{
    //         println!("{num}");
    //         sum += num.parse::<i32>().unwrap();
    //     }
    // }
    // println!("{}",sum);
}
