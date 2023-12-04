struct CardGame {
    name: u32,
    winning_numbers: Vec<u32>,
    pool_numbers: Vec<u32>,
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    let sum = input
        .lines()
        .map(|line| extract_card_games(line))
        .fold(0, |acc: u32, game| {
            let win = game
                .winning_numbers
                .iter()
                .filter(|&x| game.pool_numbers.contains(x))
                .count() as u32;
            return acc + if win == 0 { 0 } else { u32::pow(2, win - 1) };
        });
    return sum;
}

fn extract_card_games(line: &str) -> CardGame {
    let numbers = line
        .trim()
        .split([':', '|'].as_ref())
        .map(|value| string_to_vnumbers(value))
        .collect::<Vec<Vec<u32>>>();
    return CardGame {
        name: numbers[0][0],
        winning_numbers: numbers[1].clone(),
        pool_numbers: numbers[2].clone(),
    };
}

fn string_to_vnumbers(input: &str) -> Vec<u32> {
    let result;
    if input.starts_with("Card ") {
        result = input
            .split_whitespace()
            .skip(1)
            .map(|value| value.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
    } else {
        result = input
            .split_whitespace()
            .map(|value| value.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
    };
    return result;
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let result = process(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 13);
    }
}
