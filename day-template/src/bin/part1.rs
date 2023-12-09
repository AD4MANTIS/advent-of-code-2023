lib::day!("", part1, test => 0);

#[allow(unused_variables)]
fn part1(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(include_str!("./test-input.txt"));
        assert_eq!(result, 4);
    }
}
