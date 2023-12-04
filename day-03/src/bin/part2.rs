#[derive(Copy, Clone)]
struct NumberScheme {
    number: u32,
    line: u32,
    indexes: [u32; 2],
}

#[derive(Copy, Clone)]
struct SymbolScheme {
    value: char,
    line: u32,
    indexe: u32,
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    let mut sum: u32 = 0;
    let mut number_array: Vec<NumberScheme> = Vec::new();
    let mut symbol_array: Vec<SymbolScheme> = Vec::new();
    for (lindex, line) in input.lines().enumerate() {
        let mut buffer: String = String::new();
        let mut bufdex: u32 = 0;
        let mut enddex: u32 = 0;
        for (chardex, char) in line.trim().char_indices() {
            enddex = chardex as u32;
            match char {
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    if buffer.is_empty() {
                        bufdex = chardex as u32;
                    }
                    buffer.push(char);
                }
                '*' => {
                    if !buffer.is_empty() {
                        number_array.push(NumberScheme {
                            number: buffer.parse::<u32>().unwrap(),
                            line: lindex as u32,
                            indexes: [bufdex, enddex],
                        });
                        buffer.clear();
                    }
                    symbol_array.push(SymbolScheme {
                        value: char,
                        line: lindex as u32,
                        indexe: enddex,
                    });
                }
                _ => {
                    if !buffer.is_empty() {
                        number_array.push(NumberScheme {
                            number: buffer.parse::<u32>().unwrap(),
                            line: lindex as u32,
                            indexes: [bufdex, enddex],
                        });
                        buffer.clear();
                    }
                }
            }
        }
        if !buffer.is_empty() {
            number_array.push(NumberScheme {
                number: buffer.parse::<u32>().unwrap(),
                line: lindex as u32,
                indexes: [bufdex, enddex + 1],
            });
            buffer.clear();
        }
    }

    for scheme in symbol_array {
        println!(
            "Symbol: {} on line {} and with indexe {}",
            scheme.value, scheme.line, scheme.indexe
        );

        let mut collect_all = Vec::new();
        //check on previous line
        if scheme.line > 0 {
            let line = scheme.line - 1;
            let position_1 = if scheme.indexe == 0 {
                0
            } else {
                scheme.indexe - 1
            };
            let position_2 = scheme.indexe;
            let position_3 = scheme.indexe + 1;

            let collect = number_array
                .iter()
                .filter(|scheme| {
                    return scheme.line == line
                        && (is_between(scheme.indexes[0], scheme.indexes[1], position_1)
                            || is_between(scheme.indexes[0], scheme.indexes[1], position_2)
                            || is_between(scheme.indexes[0], scheme.indexes[1], position_3));
                })
                .inspect(|value| {
                    println!(
                        "Number: {} on line {} and with indexes {} to {}",
                        value.number, value.line, value.indexes[0], value.indexes[1]
                    )
                })
                .collect::<Vec<&NumberScheme>>();

            for element in collect {
                collect_all.push(element.clone());
            }
        }

        //check on same line
        {
            let line = scheme.line;
            let position_1 = if scheme.indexe == 0 {
                0
            } else {
                scheme.indexe - 1
            };
            let position_3 = scheme.indexe + 1;

            let collect = number_array
                .iter()
                .filter(|scheme| {
                    return scheme.line == line
                        && (is_between(scheme.indexes[0], scheme.indexes[1], position_1)
                            || is_between(scheme.indexes[0], scheme.indexes[1], position_3));
                })
                .inspect(|value| {
                    println!(
                        "Number: {} on line {} and with indexes {} to {}",
                        value.number, value.line, value.indexes[0], value.indexes[1]
                    )
                })
                .collect::<Vec<&NumberScheme>>();

            for element in collect {
                collect_all.push(element.clone());
            }
        }

        //check on next line
        {
            let line = scheme.line + 1;
            let position_1 = if scheme.indexe == 0 {
                0
            } else {
                scheme.indexe - 1
            };
            let position_2 = scheme.indexe;
            let position_3 = scheme.indexe + 1;

            let collect = number_array
                .iter()
                .filter(|scheme| {
                    return scheme.line == line
                        && (is_between(scheme.indexes[0], scheme.indexes[1], position_1)
                            || is_between(scheme.indexes[0], scheme.indexes[1], position_2)
                            || is_between(scheme.indexes[0], scheme.indexes[1], position_3));
                })
                .inspect(|value| {
                    println!(
                        "Number: {} on line {} and with indexes {} to {}",
                        value.number, value.line, value.indexes[0], value.indexes[1]
                    )
                })
                .collect::<Vec<&NumberScheme>>();

            for element in collect {
                collect_all.push(element.clone());
            }
        }

        if collect_all.len() == 2 {
            sum += collect_all[0].number * collect_all[1].number;
        }
    }

    return sum;
}

fn is_between(left: u32, right: u32, value: u32) -> bool {
    left <= value && right - 1 >= value
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let result = process(
            "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..",
        );
        assert_eq!(result, 467835);
    }
}
