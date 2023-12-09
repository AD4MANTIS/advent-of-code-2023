use std::{cell::RefCell, collections::HashMap, hash::Hash};

// 14_935_034_899_483
lib::day!("08", part2, test => 2);

#[derive(Debug)]
struct Node {
    name: String,

    children: Vec<String>,
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.children == other.children
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

fn part2(input: &str) -> usize {
    let steps = input.lines().collect::<Vec<_>>()[0].chars().cycle();

    let nodes_with_children = input
        .lines()
        .skip(2)
        .map(|line| {
            let name = line.split(' ').next().unwrap();
            (
                Node {
                    name: name.to_owned(),
                    children: line[(line.find('(').unwrap() + 1)..line.find(')').unwrap()]
                        .split(", ")
                        .map(str::to_owned)
                        .collect(),
                },
                RefCell::new((None::<&Node>, None::<&Node>)),
            )
        })
        .collect::<HashMap<_, _>>();
    {
        let nodes = nodes_with_children
            .keys()
            .map(|node| (node.name.clone(), node))
            .collect::<HashMap<_, _>>();

        for node in nodes_with_children.iter() {
            node.1.borrow_mut().0 = nodes.get(&node.0.children[0]).copied();
            node.1.borrow_mut().1 = nodes.get(&node.0.children[1]).copied();
        }
    }

    let mut current_nodes = nodes_with_children
        .keys()
        .filter(|node| node.name.ends_with('A'))
        .collect::<Vec<_>>();

    let mut step_count: usize = 0;
    for step in steps {
        step_count += 1;

        for current_node in current_nodes.iter_mut() {
            let children = nodes_with_children
                .get(current_node)
                .expect("current node must exist")
                .borrow();

            *current_node = match step {
                'L' => children.0,
                'R' => children.1,
                _ => panic!("directions need to be 'L' or 'R'"),
            }
            .expect("all child nodes should have been set");
        }

        if step_count % 10_000_000 == 0 {
            println!("{}M", step_count / 1_000_000);
        }

        if current_nodes.iter().all(|node| node.name.ends_with('Z')) {
            break;
        }
    }

    step_count
}
