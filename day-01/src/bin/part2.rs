enum StringOrNumber {
    StringValue(Option<String>),
    Number(u32),
}

const DIGIT_ARRAY: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut first_digit: Option<u32> = None;
        let mut last_digit = 0;
        let mut buffer: Option<String> = None;
        for char in line.chars() {
            match next_token(buffer.clone(), char) {
                StringOrNumber::Number(value) => {
                    if first_digit.is_none() {
                        first_digit = Some(value);
                    }
                    last_digit = value;
                    buffer = Some(char.to_string());
                }
                StringOrNumber::StringValue(value) => {
                    buffer = value;
                }
            }
        }
        println!(
            "Line: {}, value {}{}",
            line,
            first_digit.unwrap_or_default(),
            last_digit
        );
        sum = sum + first_digit.unwrap_or_default() * 10 + last_digit;
    }
    return sum;
}

fn next_token(buffer: Option<String>, input_char: char) -> StringOrNumber {
    if let Some(digit) = input_char.to_digit(10) {
        return StringOrNumber::Number(digit);
    } else {
        let new_buffer = buffer.unwrap_or_default() + &input_char.to_string();
        let modified_str = new_buffer[1..].to_string();
        // println!("buffer: {}", new_buffer);

        let matching_indexes: Vec<usize> = DIGIT_ARRAY
            .iter()
            .enumerate()
            .filter(|(_, &s)| s.starts_with(&new_buffer))
            .map(|(i, _)| i)
            .collect();

        let matching_mod_indexes: Vec<usize> = DIGIT_ARRAY
            .iter()
            .enumerate()
            .filter(|(_, &s)| s.starts_with(&modified_str))
            .map(|(i, _)| i)
            .collect();

        if matching_indexes.is_empty() {
            if matching_mod_indexes.is_empty() {
                return StringOrNumber::StringValue(Some(input_char.to_string()));
            } else {
                return StringOrNumber::StringValue(Some(modified_str));
            }
        } else if matching_indexes.len() == 1 {
            if DIGIT_ARRAY[matching_indexes[0]] == new_buffer {
                return StringOrNumber::Number((matching_indexes[0] + 1).try_into().unwrap());
            } else {
                // println!("buffer: {}, index: {}", new_buffer, matching_indexes[0]);
                return StringOrNumber::StringValue(Some(new_buffer));
            }
        } else {
            return StringOrNumber::StringValue(Some(new_buffer));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let result = process(
            "two1nine
          eightwothree
          abcone2threexyz
          xtwone3four
          4nineeightseven2
          zoneight234
          7pqrstsixteen",
        );
        assert_eq!(result, 281);
    }
}
