fn main() {
    let input = include_str!("./input1.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    "todo!".to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let result = process("test");
        assert_eq!(result, "4".to_string());
    }
}
