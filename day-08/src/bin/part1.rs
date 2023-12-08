use std::{cell::RefCell, collections::HashMap, hash::Hash};

fn main() {
    let _timer = lib::PrintTimer::new("");

    let input = include_str!("./input.txt");
    let output = part1(input);

    dbg!(output);
}

#[derive(Debug)]
struct Node {
    name: String,

    childs: Vec<String>,
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.childs == other.childs
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

fn part1(input: &str) -> usize {
    let steps = input.lines().collect::<Vec<_>>()[0].chars().cycle();

    let nodes_with_childs = input
        .lines()
        .skip(2)
        .map(|line| {
            let name = line.split(' ').next().unwrap();
            (
                Node {
                    name: name.to_owned(),
                    childs: line[(line.find('(').unwrap() + 1)..line.find(')').unwrap()]
                        .split(", ")
                        .map(str::to_owned)
                        .collect(),
                },
                RefCell::new((None::<&Node>, None::<&Node>)),
            )
        })
        .collect::<HashMap<_, _>>();

    let mut current_node: &Node;
    {
        let nodes = nodes_with_childs
            .keys()
            .map(|node| (node.name.clone(), node))
            .collect::<HashMap<_, _>>();

        for node in nodes_with_childs.iter() {
            node.1.borrow_mut().0 = nodes.get(&node.0.childs[0]).copied();
            node.1.borrow_mut().1 = nodes.get(&node.0.childs[1]).copied();
        }

        current_node = nodes.get("AAA").copied().unwrap();
    }

    let mut step_count = 0;
    for step in steps {
        step_count += 1;

        let childs = nodes_with_childs.get(current_node).unwrap().borrow();
        current_node = match step {
            'L' => childs.0,
            'R' => childs.1,
            _ => panic!(),
        }
        .unwrap();

        if current_node.name == "ZZZ" {
            break;
        }
    }

    step_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(include_str!("./test-input.txt"));
        assert_eq!(result, 2);
    }
}
