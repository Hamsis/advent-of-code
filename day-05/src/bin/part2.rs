use std::collections::HashMap;

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    let mut seeds = Vec::<(u64, u64)>::new();
    let mut soil_map = Vec::<(u64, u64, u64)>::new();
    let mut fetilizer_map = Vec::<(u64, u64, u64)>::new();
    let mut water_map = Vec::<(u64, u64, u64)>::new();
    let mut light_map = Vec::<(u64, u64, u64)>::new();
    let mut temperature_map = Vec::<(u64, u64, u64)>::new();
    let mut humidity_map = Vec::<(u64, u64, u64)>::new();
    let mut location_map = Vec::<(u64, u64, u64)>::new();
    let mut line;

    let mut lines = input.lines();

    // 1- get the seeds
    line = lines.next().unwrap();
    let mut buffer: Option<u64> = None;
    for seed_number in line[7..]
        .split_whitespace()
        .map(|value| value.parse::<u64>().unwrap())
    {
        if let Some(value) = buffer {
            seeds.push((value, seed_number));
            buffer = None;
        } else {
            buffer = Some(seed_number);
        }
    }

    // 2- get the soil ref
    lines.next();
    lines.next();
    line = lines.next().unwrap().trim();
    while !line.is_empty() {
        let collect = line
            .trim()
            .split_whitespace()
            .map(|value| value.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        soil_map.push((collect[0], collect[1], collect[2]));
        line = lines.next().unwrap().trim();
    }

    // 3- get the fertilizer ref
    lines.next();
    line = lines.next().unwrap().trim();
    while !line.is_empty() {
        let collect = line
            .trim()
            .split_whitespace()
            .map(|value| value.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        fetilizer_map.push((collect[0], collect[1], collect[2]));
        line = lines.next().unwrap().trim();
    }

    // 4- get the water ref
    lines.next();
    line = lines.next().unwrap().trim();
    while !line.is_empty() {
        let collect = line
            .trim()
            .split_whitespace()
            .map(|value| value.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        water_map.push((collect[0], collect[1], collect[2]));
        line = lines.next().unwrap().trim();
    }

    // 5- get the light ref
    lines.next();
    line = lines.next().unwrap().trim();
    while !line.is_empty() {
        let collect = line
            .trim()
            .split_whitespace()
            .map(|value| value.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        light_map.push((collect[0], collect[1], collect[2]));
        line = lines.next().unwrap().trim();
    }

    // 6- get the temperature ref
    lines.next();
    line = lines.next().unwrap().trim();
    while !line.is_empty() {
        let collect = line
            .trim()
            .split_whitespace()
            .map(|value| value.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        temperature_map.push((collect[0], collect[1], collect[2]));
        line = lines.next().unwrap().trim();
    }

    // 7- get the humidity ref
    lines.next();
    line = lines.next().unwrap().trim();
    while !line.is_empty() {
        let collect = line
            .trim()
            .split_whitespace()
            .map(|value| value.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        humidity_map.push((collect[0], collect[1], collect[2]));
        line = lines.next().unwrap().trim();
    }

    // 8- get the location ref
    lines.next();
    line = lines.next().unwrap().trim();
    while !line.is_empty() {
        let collect = line
            .trim()
            .split_whitespace()
            .map(|value| value.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        location_map.push((collect[0], collect[1], collect[2]));
        if let Some(value) = lines.next() {
            line = value.trim();
        } else {
            break;
        }
    }

    let mut location: Option<u32> = None;
    for seed in seeds
        .iter()
        .flat_map(|(initial_value, range)| *initial_value..(*initial_value + *range))
    {
        let mut result;
        let mut search_value;
        let mut correspondance;
        // 1- seed to soil
        search_value = seed;
        correspondance = 0;
        result = soil_map
            .iter()
            .find(|&x| x.1 <= search_value && search_value < x.1 + x.2);
        if let Some(value) = result {
            correspondance = search_value - value.1 + value.0;
        } else {
            correspondance = search_value;
        }

        // 2- soil to fertilizer
        search_value = correspondance;
        result = fetilizer_map
            .iter()
            .find(|&x| x.1 <= search_value && search_value < x.1 + x.2);
        if let Some(value) = result {
            correspondance = search_value - value.1 + value.0;
        } else {
            correspondance = search_value;
        }

        // 3- fertilizer to water
        search_value = correspondance;
        result = water_map
            .iter()
            .find(|&x| x.1 <= search_value && search_value < x.1 + x.2);
        if let Some(value) = result {
            correspondance = search_value - value.1 + value.0;
        } else {
            correspondance = search_value;
        }

        // 4- water to light
        search_value = correspondance;
        result = light_map
            .iter()
            .find(|&x| x.1 <= search_value && search_value < x.1 + x.2);
        if let Some(value) = result {
            correspondance = search_value - value.1 + value.0;
        } else {
            correspondance = search_value;
        }

        // 5- light to temperature
        search_value = correspondance;
        result = temperature_map
            .iter()
            .find(|&x| x.1 <= search_value && search_value < x.1 + x.2);
        if let Some(value) = result {
            correspondance = search_value - value.1 + value.0;
        } else {
            correspondance = search_value;
        }

        // 6- temperature to humidity
        search_value = correspondance;
        result = humidity_map
            .iter()
            .find(|&x| x.1 <= search_value && search_value < x.1 + x.2);
        if let Some(value) = result {
            correspondance = search_value - value.1 + value.0;
        } else {
            correspondance = search_value;
        }

        // 7- humidity to location
        search_value = correspondance;
        result = location_map
            .iter()
            .find(|&x| x.1 <= search_value && search_value < x.1 + x.2);
        if let Some(value) = result {
            correspondance = search_value - value.1 + value.0;
        } else {
            correspondance = search_value;
        }

        if let Some(value) = location {
            location = Some(value.min(correspondance.try_into().unwrap()));
        } else {
            location = Some(correspondance.try_into().unwrap());
        }
    }

    println!("{:?}", seeds);
    return location.unwrap();
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let result = process(
            "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4",
        );
        assert_eq!(result, 46);
    }
}
