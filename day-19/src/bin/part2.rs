use std::{collections::HashMap, str::FromStr};

lib::day!(19, part1, example => 19114, answer => 480738);

// px{a<2006:qkq,m>2090:A,rfg}
#[derive(Debug, Clone)]
pub struct Workflow {
    name: String,
    flows: Vec<Flow>,
}

#[derive(Debug, Clone, Default)]
pub struct Flow {
    condition: Option<FlowCondition>,
    next_workflow: String,
}

#[derive(Debug, Clone)]
pub struct FlowCondition {
    category: PartCategory,
    op: String,
    rhs: usize,
}

#[derive(Debug, Clone)]
pub struct Rating {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Rating {
    pub const fn get(&self, cat: PartCategory) -> usize {
        match cat {
            PartCategory::X => self.x,
            PartCategory::M => self.m,
            PartCategory::A => self.a,
            PartCategory::S => self.s,
        }
    }

    pub fn check(&self, cond: &FlowCondition) -> bool {
        let value = self.get(cond.category);

        match cond.op.as_str() {
            ">" => value > cond.rhs,
            "<" => value < cond.rhs,
            _ => panic!(),
        }
    }

    pub const fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PartCategory {
    X, // Extremely cool looking
    M, // Musical (it makes a noise when you hit it)
    A, // Aerodynamic
    S, // Shiny
}

impl FromStr for PartCategory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => Err(())?,
        })
    }
}

fn part1(input: &'static str) -> usize {
    let [workflows, ratings]: [&'static str; 2] =
        input.split("\n\n").collect::<Vec<_>>().try_into().unwrap();

    let workflows = get_workflows(workflows);

    get_ratings(ratings)
        .into_iter()
        .filter(|rating| {
            let mut workflow = workflows.get("in").expect("start should be present");

            while !matches!(workflow.name.as_str(), "A" | "R") {
                for flow in workflow.flows.iter() {
                    if let Some(cond) = &flow.condition {
                        if !rating.check(cond) {
                            continue;
                        }
                    }

                    match workflows.get(&flow.next_workflow) {
                        Some(next_workflow) => workflow = next_workflow,
                        None => return flow.next_workflow == "A",
                    }

                    break;
                }
            }

            workflow.name == "A"
        })
        .map(|r| r.sum())
        .sum()

    // dbg!(&workflows);
    // dbg!(&ratings);
}

fn get_ratings(ratings: &str) -> Vec<Rating> {
    let ratings = ratings
        .lines()
        .map(|rating_str| {
            let ratings_str: [usize; 4] = rating_str
                .split(',')
                .flat_map(|rating| {
                    rating
                        .chars()
                        .filter(|c| char::is_numeric(*c))
                        .collect::<String>()
                        .parse::<usize>()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            Rating {
                x: ratings_str[0],
                m: ratings_str[1],
                a: ratings_str[2],
                s: ratings_str[3],
            }
        })
        .collect::<Vec<_>>();
    ratings
}

fn get_workflows(workflows: &str) -> HashMap<String, Workflow> {
    let workflows = workflows
        .lines()
        .map(|line| {
            let name = line.chars().take_while(|c| *c != '{').collect::<String>();

            Workflow {
                flows: line
                    .trim_end_matches('}')
                    .chars()
                    .skip(name.len() + 1)
                    .collect::<String>()
                    .split(',')
                    .map(|flow_str| {
                        let mut flow_str = flow_str.split(':').rev();

                        Flow {
                            next_workflow: flow_str.next().unwrap().to_string(),
                            condition: flow_str.next().map(|cond| FlowCondition {
                                category: PartCategory::from_str(&cond[0..=0]).unwrap(),
                                op: cond[1..=1].to_string(),
                                rhs: cond[2..].parse().unwrap(),
                            }),
                        }
                    })
                    .collect(),
                name,
            }
        })
        .map(|workflow| (workflow.name.clone(), workflow))
        .collect::<HashMap<_, _>>();
    workflows
}
