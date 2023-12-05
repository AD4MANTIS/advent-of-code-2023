use std::time::SystemTime;

fn main() {
    let start_time = SystemTime::now();

    let input = include_str!("./input.txt");
    let output = part1(input);

    println!(
        "Output = {output} (Duration: {})",
        start_time.elapsed().unwrap_or_default().as_secs_f64()
    );
}

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
