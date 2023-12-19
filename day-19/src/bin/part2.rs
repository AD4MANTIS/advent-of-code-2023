#![allow(unused)]

use std::{iter, str::FromStr};

// lib::day!(19, part2, example => 167409079868000);

fn main() {}

// px{a<2006:qkq,m>2090:A,rfg}
#[derive(Debug, Clone)]
pub struct Workflow {
    name: u32,
    flows: Vec<Flow>,
    next_default_workflow: u32,
}

#[derive(Debug, Clone)]
pub struct Flow {
    condition: FlowCondition,
    next_workflow: u32,
}

#[derive(Debug, Clone)]
pub struct FlowCondition {
    /// See [PartCategory]
    category: u8,
    op: Operation,
    rhs: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Less,
    Greater,
}

// #[derive(Debug, Clone)]
// pub struct Rating {
//     x: u16,
//     m: u16,
//     a: u16,
//     s: u16,
// }

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

fn part2(input: &'static str) -> usize {
    let [workflows, _ratings]: [&'static str; 2] =
        input.split("\n\n").collect::<Vec<_>>().try_into().unwrap();

    let workflows = get_workflows(workflows);

    let mut count = 0_usize;
    const IN_KEY: u32 = 28265; // hash("in");
    const A_KEY: u32 = 65; //hash("A");
    const R_KEY: u32 = 82; //hash("R");

    for x in 1..=4000 {
        dbg!(x);
        for m in 1..=4000 {
            dbg!(m);

            for a in 1..=4000 {
                for s in 1..=4000 {
                    let rating = [x, m, a, s];

                    let mut workflow = workflows[IN_KEY as usize].as_ref().unwrap();

                    'workflow_loop: while workflow.name != A_KEY && workflow.name != R_KEY {
                        for flow in workflow.flows.iter() {
                            let value = rating[flow.condition.category as usize];

                            if match flow.condition.op {
                                Operation::Greater => value > flow.condition.rhs,
                                Operation::Less => value < flow.condition.rhs,
                            } {
                                workflow = workflows[flow.next_workflow as usize].as_ref().unwrap();
                                break 'workflow_loop;
                            }
                        }

                        // if workflow.next_default_workflow == a_key {
                        //     count += 1;
                        // }

                        workflow = workflows[workflow.next_default_workflow as usize]
                            .as_ref()
                            .unwrap();
                    }

                    if workflow.name == A_KEY {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn hash(name: &str) -> u32 {
    let mut chars = name.chars();
    (chars.next().unwrap() as u8 as u32)
        + ((chars.next().unwrap_or_default() as u8 as u32) << 8)
        + ((chars.next().unwrap_or_default() as u8 as u32) << 16)
}

fn get_workflows(workflows: &str) -> Vec<Option<Workflow>> {
    let mut arr: Vec<Option<Workflow>> = iter::repeat(None::<Workflow>).take(8024434).collect();

    workflows
        .lines()
        .map(|line| {
            let name = line.chars().take_while(|c| *c != '{').collect::<String>();

            let mut next_default_workflow = 0;

            let flows = line
                .trim_end_matches('}')
                .chars()
                .skip(name.len() + 1)
                .collect::<String>()
                .split(',')
                .flat_map(|flow_str| {
                    let mut flow_str = flow_str.split(':').rev();

                    let next_workflow = hash(flow_str.next().unwrap());

                    if let Some(cond) = flow_str.next().map(|cond| FlowCondition {
                        category: match &cond[0..=0] {
                            "x" => 0,
                            "m" => 1,
                            "a" => 2,
                            "s" => 3,
                            _ => panic!(),
                        },
                        op: match &cond[1..=1] {
                            "<" => Operation::Less,
                            ">" => Operation::Greater,
                            _ => panic!(),
                        },
                        rhs: cond[2..].parse().unwrap(),
                    }) {
                        return Some(Flow {
                            next_workflow,
                            condition: cond,
                        });
                    }

                    next_default_workflow = next_workflow;

                    None
                })
                .collect();

            Workflow {
                name: hash(&name),
                flows,
                next_default_workflow,
            }
        })
        .for_each(|workflow| arr[workflow.name as usize] = Some(workflow.clone()));

    arr
}
