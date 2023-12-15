lib::day!(15, part1, example => 1320, answer => 518107);

fn part1(input: &str) -> u64 {
    input.split(',').map(hash).sum()
}

fn hash(val: &str) -> u64 {
    let mut hash = 0u64;

    for char in val.chars() {
        // Determine the ASCII code for the current character of the string.
        // Increase the current value by the ASCII code you just determined.
        hash += char as u64;

        // Set the current value to itself multiplied by 17.
        hash *= 17;

        // Set the current value to the remainder of dividing itself by 256.
        hash %= 256;
    }

    hash
}
