fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut first_digit: Option<u32> = None;
        let mut last_digit = 0;
        for char in line.chars() {
            if let Some(digit) = char.to_digit(10) {
                if first_digit.is_none() {
                    first_digit = Some(digit);
                }
                last_digit = digit;
            }
        }
        println!(
            "Line: {}, value {}{}",
            line,
            first_digit.unwrap(),
            last_digit
        );
        sum = sum + first_digit.unwrap_or_default() * 10 + last_digit;
    }
    return sum;
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let result = process(
            "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet",
        );
        assert_eq!(result, 142);
    }
}
