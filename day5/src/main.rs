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
    println!("seeds {:?}", seeds.first().unwrap());
    let mut sources: Vec<i64> = Vec::new();
    for seed in seeds.first().unwrap(){
        let mut source: i64 = seed.parse().unwrap();
        println!("Source: {:?}", source);
        for (label, maps) in input[1..].iter(){
            println!("{} {:?}", label, maps);
            for map in maps.iter(){
                println!("{} {} {}", map[0], map[1], map[2]);
                let map_destination: i64= map[0].clone().parse().unwrap();
                let map_source: i64 = map[1].clone().parse().unwrap();
                let map_range: i64 =map[2].clone().parse().unwrap();
                let mut destination_expansion: Vec<i64> = Vec::new();
                let mut source_expansion: Vec<i64> = Vec::new();
                // println!("Destination:{}, Source:{}, range:{}", map_destination, map_source, map_range);
                for i in 0..map_range{
                    destination_expansion.push(map_destination+i);
                    source_expansion.push(map_source+i);
                }
                if source_expansion.contains(&source){
                    source = destination_expansion[source_expansion.iter().position(|el| *el == source ).unwrap()];
                    break;
                }
                // println!("{:?}{:?}", destination_expansion, source_expansion);
            }
        }
        sources.push(source);
    }
    println!("Locations {:?}", sources);
    println!("Nearest location {:?}", sources.iter().min().unwrap());
}
