use std::fs;

fn recurse(ref_serie: &Vec<i32>)->i32{
    let mut diff_serie: Vec<i32> = Vec::new();
    let mut current = 0;
    let mut next = 1;
    while next < ref_serie.len(){   
        diff_serie.push(ref_serie[next as usize] - ref_serie[current as usize]);
        current += 1;
        next += 1;
    }
    println!("{:?}", diff_serie);
    // if !diff_serie.iter().all(|diff| *diff == 0){
    if diff_serie.iter().all(|diff| *diff == 0){
        // recurse(&diff_serie);
        return 0;
    }
    return diff_serie.iter().last().unwrap() + recurse(&diff_serie);
}

fn rev_recurse(ref_serie: &Vec<i32>)->i32{
    let mut diff_serie: Vec<i32> = Vec::new();
    let mut current = 0;
    let mut next = 1;
    while next < ref_serie.len(){   
        diff_serie.push(ref_serie[next as usize] - ref_serie[current as usize]);
        current += 1;
        next += 1;
    }
    println!("{:?}", diff_serie);
    // if !diff_serie.iter().all(|diff| *diff == 0){
    if diff_serie.iter().all(|diff| *diff == 0){
        // recurse(&diff_serie);
        println!("{}",0);
        return 0;
    }
    let ret_val =diff_serie[0] - rev_recurse(&diff_serie); 
    println!("{}",ret_val);
    return ret_val;
}
fn main() { 
    // let path = "src/test_p1.txt";
    let path = "src/p1.txt";
    let input: Vec<String> = fs::read_to_string(path).expect("Error reading file.").lines().map(|line| line.to_string()).collect();
    let series: Vec<Vec<i32>> = input.iter().map(|line| line.split_whitespace().map(|num| num.parse().unwrap()).collect()).collect();
    // println!("{:?}", input);
    // println!("{:?}", series);
    let mut total = 0;
    for serie in series.iter(){
        // let return_val = serie.iter().last().unwrap() + recurse(serie);
        let return_val = serie[0] - rev_recurse(serie);
        total += return_val;
        println!("{:?}", return_val);
    }
    println!("{:?}", total);
}
