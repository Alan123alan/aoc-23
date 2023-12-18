use std::fs;

fn main() {
    const PATH: &str = "./src/input_day5_p1.txt";
    let input: Vec<(String, Vec<Vec<String>>)> = fs::read_to_string(PATH).
        expect("Failed to open file.")
        .split("\n\n")
        // .into_iter()
        .map(|block| block.split_once(":").unwrap())
        .map(|(label, values)| (label.trim().to_owned(), values.trim().lines().map(|line| line.split_whitespace().map(|num| num.to_owned()).collect::<Vec<String>>()).collect::<Vec<Vec<String>>>()))
        .collect();
    let(_, seeds) = input[0].clone();
    let mut seeds_expansion: Vec<Vec<i64>> = Vec::new();
    let len = seeds.first().unwrap().len();
    for index in (0..len).into_iter().step_by(2){
        println!("{}", index);
        let start: i64 = seeds.first().unwrap()[index].parse().unwrap(); 
        let range: i64 = seeds.first().unwrap()[index+1].parse().unwrap();
        let end: i64 = start + range - 1;
        println!("start: {}, range: {}, end: {}", start, range, end);
        seeds_expansion.push((start..end).into_iter().collect())

    }
    // println!("seeds_expansion {:?}", seeds_expansion);
    println!("seeds_expansion length {:?}", seeds_expansion.len());
    println!("seeds_expansion first expansion length {:?}", seeds_expansion[0].len());
    // println!("seeds {:?}", seeds.first().unwrap());
    // let mut sources: Vec<i64> = Vec::new();
    // for seed in seeds.first().unwrap(){
    for (current_expansion, seed_expansion) in seeds_expansion.iter().enumerate(){
        println!("{}", current_expansion);
        for seed in seed_expansion{
            // let mut source: i64 = seed.parse().unwrap();
            let mut source: i64 = *seed;
            // println!("Source: {:?}", source);
            for (label, maps) in input[1..].iter(){
                // println!("{} {:?}", label, maps);
                for map in maps.iter(){
                    // println!("{} {} {}", map[0], map[1], map[2]);
                    let map_destination: i64= map[0].clone().parse().unwrap();
                    let map_source: i64 = map[1].clone().parse().unwrap();
                    let map_range: i64 =map[2].clone().parse().unwrap();
                    if source >= map_source && source <= map_source + map_range - 1{
                        source = map_destination + (source - map_source);
                        break;
                    }
                            // println!("{}{}{}", map_destination, map_source, map_range);
                }
            }
            // sources.push(source);
        }
    }
    // println!("Locations {:?}", sources);
    // println!("Nearest location {:?}", sources.iter().min().unwrap());
}
