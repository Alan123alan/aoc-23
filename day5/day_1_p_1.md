You get a list of seeds and a series of maps

Each map purpose is to describe how to convert numbers from a **source category** into numbers in a **destination category**



The almanac starts by listing which seeds need to be planted: seeds 79, 14, 55, and 13.

The rest of the almanac contains a list of maps which describe how to convert numbers from a source category into numbers in a destination category. That is, the section that starts with seed-to-soil map: describes how to convert a seed number (the source) to a soil number (the destination). This lets the gardener and his team know which soil to use with which seeds, which water to use with which fertilizer, and so on.

Rather than list every source number and its corresponding destination number one by one, the maps describe entire ranges of numbers that can be converted. Each line within a map contains three numbers: the destination range start, the source range start, and the range length.

Consider again the example seed-to-soil map:

50 98 2
52 50 48

The first line has a destination range start of 50, a source range start of 98, and a range length of 2. This line means that the source range starts at 98 and contains two values: 98 and 99. The destination range is the same length, but it starts at 50, so its two values are 50 and 51. With this information, you know that seed number 98 corresponds to soil number 50 and that seed number 99 corresponds to soil number 51.

The second line means that the source range starts at 50 and contains 48 values: 50, 51, ..., 96, 97. This corresponds to a destination range starting at 52 and also containing 48 values: 52, 53, ..., 98, 99. So, seed number 53 corresponds to soil number 55.

Any source numbers that aren't mapped correspond to the same destination number. So, seed number 10 corresponds to soil number 10.

So, the entire list of seed numbers and their corresponding soil numbers looks like this:

seed  soil
0     0
1     1
...   ...
48    48
49    49
50    52
51    53
...   ...
96    98
97    99
98    50
99    51

With this map, you can look up the soil number required for each initial seed number:

Seed number 79 corresponds to soil number 81.
Seed number 14 corresponds to soil number 14.
Seed number 55 corresponds to soil number 57.
Seed number 13 corresponds to soil number 13.
The gardener and his team want to get started as soon as possible, so they'd like to know the closest location that needs a seed. Using these maps, find the lowest location number that corresponds to any of the initial seeds. To do this, you'll need to convert each seed number through other categories until you can find its corresponding location number. In this example, the corresponding types are:

Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.
So, the lowest location number in this example is 35.

What is the lowest location number that corresponds to any of the initial seed numbers

iterate through every seed given and initialize the source to the value seed
every seed gets mapped by every map from source to a new destination
update the source to the destination after each new mapping

will avoid expansion of the range of sources and destinations reduce processing time?
every time the vec expands out of capacity it is suppossed to allocate more memory, is this the main issue?

program executes in a few seconds after removing this code:
```Rust

for i in 0..map_range{
    destination_expansion.push(map_destination+i);
    source_expansion.push(map_source+i);
}
 
```

total positions goes from 0 to map_range-1

if source  exists within the (map_source) <-> (map_source + map_range -1)

you should be able to get the difference/ index/ #steps by subtracting the map_source from the source

example

| Destination | Source | Range |
| -- | -- | -- |
| 52 | 50 | 48 |

and assume our seed is 56
we set our source to the seed value
check if the source exists within the 50-97 range (it does)
subtract 56 - 50 and get the difference/index/#steps = 6
add 6 + 52 and get the destination mapping = 58
update the source to the destination mapping obtained

Does this work in practice?


