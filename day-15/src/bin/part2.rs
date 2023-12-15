use std::{collections::HashMap, hash::Hash};

lib::day!(15, part2, example => 145, answer => 303404);

fn part2(input: &str) -> u64 {
    let mut boxes: HashMap<u64, Vec<Lens>> = (0..256).map(|id| (id, Vec::new())).collect();

    for operation in input.split(',') {
        let lens = Lens::from_string(operation);

        boxes.entry(hash(&lens.label)).and_modify(|b| {
            let pos = b
                .iter()
                .position(|lens_in_box| lens_in_box.label == lens.label);

            match lens.operation {
                '-' => {
                    if let Some(pos) = pos {
                        b.remove(pos);
                    }
                }
                '=' => {
                    if let Some(pos) = pos {
                        b[pos] = lens;
                    } else {
                        b.push(lens);
                    }
                }
                _ => panic!(),
            };
        });
    }

    boxes
        .iter()
        .filter(|x| !x.1.is_empty())
        .flat_map(|lens_box| {
            lens_box
                .1
                .iter()
                .enumerate()
                .map(move |lens| (lens_box.0 + 1) * (lens.0 as u64 + 1) * lens.1.focal_length)
        })
        .sum()
}

#[derive(Debug)]
struct Lens {
    label: String,
    operation: char,
    focal_length: u64,
}

impl Lens {
    pub fn from_string(val: &str) -> Self {
        let label = val
            .chars()
            .take_while(|c| *c >= 'a' && *c <= 'z')
            .collect::<String>();

        Self {
            operation: val.chars().nth(label.len()).unwrap(),
            label,
            focal_length: val
                .split('=')
                .last()
                .and_then(|focal_length| str::parse(focal_length).ok())
                .unwrap_or_default(),
        }
    }
}

struct Label<'a>(&'a str);

impl<'a> Hash for Label<'a> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(hash(self.0));
    }
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
