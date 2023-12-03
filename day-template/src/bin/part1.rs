fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[allow(unused_variables)]
fn part1(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(include_str!("./test-input.txt"));
        assert_eq!(result, 4.to_string());
    }
}
