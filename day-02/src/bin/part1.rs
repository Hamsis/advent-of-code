use std::result;

const RED_CUBES: u32 = 12;
const GREEN_CUBES: u32 = 13;
const BLUE_CUBES: u32 = 14;

const RGB_ARRAY: [&str; 3] = ["red", "green", "blue"];

struct RGBValue {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        println!("{}", line);
        let game_number = line[line.find('e').unwrap() + 2..line.find(':').unwrap()]
            .parse::<u32>()
            .unwrap();
        let result_sets = &line[line.find(':').unwrap() + 1..];
        let collect = result_sets
            .split(';')
            // .inspect(|result| println!("{}", result))
            .map(|set| get_rgb_value(set.trim()))
            // .inspect(|result| println!("RGB {} {} {}", result.red, result.green, result.blue))
            .filter(|result| {
                return result.red > RED_CUBES
                    || result.green > GREEN_CUBES
                    || result.blue > BLUE_CUBES;
            })
            .count();
        println!("Sets in error {}", collect);
        if collect == 0 {
            sum = sum + game_number;
        }
    }
    return sum;
}

fn get_rgb_value(set_result: &str) -> RGBValue {
    let mut red_value = 0;
    let mut green_value = 0;
    let mut blue_value = 0;

    let groups = set_result
        .split(',')
        .map(|value| value.trim())
        .collect::<Vec<&str>>();
    for group in groups {
        let inner_grp = group.split(' ').collect::<Vec<&str>>();
        let color_index = RGB_ARRAY
            .iter()
            .position(|item| item.to_string() == inner_grp[1].to_string());

        if color_index.is_some() {
            match color_index.unwrap() {
                0 => {
                    red_value = inner_grp[0].parse::<u32>().unwrap();
                }
                1 => {
                    green_value = inner_grp[0].parse::<u32>().unwrap();
                }
                2 => {
                    blue_value = inner_grp[0].parse::<u32>().unwrap();
                }
                _ => {}
            }
        }
    }

    return RGBValue {
        red: red_value,
        green: green_value,
        blue: blue_value,
    };
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let result = process(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 8);
    }
}
