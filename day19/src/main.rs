#[derive(Debug, Clone)]
struct Rule {
    member: String,
    ordering: std::cmp::Ordering,
    value: usize,
    target: String,
}

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rule>,
    target: String,
}

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

fn main() {
    let lines = std::io::stdin().lines().flatten().collect::<Vec<_>>();

    let workflows = {
        lines
            .iter()
            .filter_map(|l| l.split_once('{'))
            .filter(|(name, _)| !name.is_empty())
            .map(|(name, rules)| {
                let mut rules = rules
                    .strip_suffix('}')
                    .unwrap()
                    .split(',')
                    .collect::<Vec<_>>();
                let target = rules.pop().unwrap().to_string();
                let rules = rules
                    .iter()
                    .map(|rule| {
                        let member = rule.chars().next().unwrap().to_string();
                        let ordering = match rule.chars().nth(1).unwrap() {
                            '<' => std::cmp::Ordering::Less,
                            '>' => std::cmp::Ordering::Greater,
                            _ => unreachable!(),
                        };
                        let (value, target) = rule[2..].split_once(':').unwrap();
                        Rule {
                            member,
                            ordering,
                            value: value.parse::<usize>().unwrap(),
                            target: target.to_string(),
                        }
                    })
                    .collect();

                (name.to_string(), Workflow { rules, target })
            })
    }
    .collect::<std::collections::HashMap<_, _>>();

    let parts = {
        lines
            .iter()
            .filter_map(|l| l.split_once('{'))
            .filter(|(name, _)| name.is_empty())
            .map(|(_, part)| {
                let attrs = part
                    .strip_suffix('}')
                    .unwrap()
                    .split(',')
                    .map(|attr| attr[2..].parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                Part {
                    x: attrs[0],
                    m: attrs[1],
                    a: attrs[2],
                    s: attrs[3],
                }
            })
            .collect::<Vec<_>>()
    };

    let result = parts
        .iter()
        .filter(|part| is_good(part, &workflows))
        .map(|part| part.x + part.m + part.a + part.s)
        .sum::<usize>();

    println!("{result}");

    let result = std::iter::successors(
        Some(vec![(
            vec![1, 1, 1, 1],
            vec![4000, 4000, 4000, 4000],
            workflows.get("in").unwrap().clone(),
        )]),
        |stack| {
            let mut stack = stack.clone();
            match stack.pop() {
                None => None,
                Some((mins, maxs, workflow)) => {
                    let mut rules = workflow.rules.iter();
                    let rule = rules.next();
                    match rule {
                        None => {
                            if workflow.target != "A" && workflow.target != "R" {
                                stack.push((
                                    mins,
                                    maxs,
                                    workflows.get(&workflow.target).unwrap().clone(),
                                ));
                            }
                            Some(stack.to_vec())
                        }
                        Some(rule) => {
                            let (rule_mins, rule_maxs, workflow_mins, workflow_maxs) =
                                partition_ranges(&mins, &maxs, rule);
                            stack.push((
                                workflow_mins,
                                workflow_maxs,
                                Workflow {
                                    rules: rules.cloned().collect::<Vec<_>>(),
                                    target: workflow.target.clone(),
                                },
                            ));
                            if rule.target != "A" && rule.target != "R" {
                                stack.push((
                                    rule_mins,
                                    rule_maxs,
                                    workflows.get(&rule.target).unwrap().clone(),
                                ));
                            }
                            Some(stack.to_vec())
                        }
                    }
                }
            }
        },
    )
    .map(|stack| match stack.iter().last() {
        Some((mins, maxs, workflow)) => match workflow.rules.len() {
            0 => match workflow.target.as_ref() {
                "A" => perms(mins, maxs),
                _ => 0,
            },
            _ => match workflow.rules[0].target.as_ref() {
                "A" => {
                    let (mins, maxs, _, _) = partition_ranges(mins, maxs, &workflow.rules[0]);
                    perms(&mins, &maxs)
                }
                _ => 0,
            },
        },
        None => 0,
    })
    .sum::<i64>();

    println!("{}", result);
}

fn is_good(part: &Part, workflows: &std::collections::HashMap<String, Workflow>) -> bool {
    std::iter::successors(Some("in"), |target| match *target {
        "R" => None,
        "A" => None,
        target => Some({
            let workflow = workflows.get(target).unwrap();
            workflow
                .rules
                .iter()
                .find_map(|rule| {
                    let value = match rule.member.as_ref() {
                        "x" => part.x,
                        "m" => part.m,
                        "a" => part.a,
                        "s" => part.s,
                        _ => unreachable!(),
                    };

                    match rule.ordering {
                        std::cmp::Ordering::Less if value < rule.value => {
                            Some(rule.target.as_ref())
                        }
                        std::cmp::Ordering::Greater if value > rule.value => {
                            Some(rule.target.as_ref())
                        }
                        _ => None,
                    }
                })
                .unwrap_or(workflow.target.as_ref())
        }),
    })
    .last()
    .unwrap()
        == "A"
}

fn partition_ranges(
    mins: &[usize],
    maxs: &[usize],
    rule: &Rule,
) -> (Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>) {
    let mut rule_mins = mins.to_vec();
    let mut rule_maxs = maxs.to_vec();
    let mut workflow_mins = mins.to_vec();
    let mut workflow_maxs = maxs.to_vec();

    let index = match rule.member.as_str() {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => unreachable!(),
    };

    rule_mins[index] = match rule.ordering {
        std::cmp::Ordering::Less => rule_mins[index],
        std::cmp::Ordering::Greater => (rule.value + 1).max(rule_mins[index]),
        _ => unreachable!(),
    };

    rule_maxs[index] = match rule.ordering {
        std::cmp::Ordering::Less => (rule.value - 1).min(rule_maxs[index]),
        std::cmp::Ordering::Greater => rule_maxs[index],
        _ => unreachable!(),
    };

    workflow_mins[index] = match rule.ordering {
        std::cmp::Ordering::Less => rule.value.max(workflow_mins[index]),
        std::cmp::Ordering::Greater => workflow_mins[index],
        _ => unreachable!(),
    };

    workflow_maxs[index] = match rule.ordering {
        std::cmp::Ordering::Less => workflow_maxs[index],
        std::cmp::Ordering::Greater => rule.value.min(workflow_maxs[index]),
        _ => unreachable!(),
    };

    (rule_mins, rule_maxs, workflow_mins, workflow_maxs)
}

fn perms(mins: &[usize], maxs: &[usize]) -> i64 {
    mins.iter()
        .zip(maxs.iter())
        .map(|(min, max)| (max - min + 1) as i64)
        .product()
}
