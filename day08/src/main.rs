#[derive(Debug)]
struct Cycle<'a> {
    key: &'a str,
    start: Option<usize>,
    period: Option<usize>,
}

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();

    let directions = &lines[0];
    let waypoints = lines[2..]
        .iter()
        .map(|l| l.split_once(" = ").unwrap())
        .map(|(key, lr)| (key, lr[1..9].split_once(", ").unwrap()))
        .collect::<std::collections::HashMap<_, _>>();

    //    let result = directions
    //        .chars()
    //        .cycle()
    //        .scan("AAA", |waypoint, direction| {
    //            *waypoint = match direction {
    //                'L' => waypoints[waypoint].0,
    //                _ => waypoints[waypoint].1,
    //            };
    //            match *waypoint {
    //                "ZZZ" => None,
    //                _ => Some(()),
    //            }
    //        })
    //        .count()
    //        + 1;
    //    println!("{}", result);

    let cycles = directions
        .chars()
        .cycle()
        .enumerate()
        .scan(
            waypoints
                .keys()
                .filter(|k| k.chars().nth(2).unwrap() == 'A')
                .map(|k| Cycle {
                    key: k,
                    start: None,
                    period: None,
                })
                .collect::<Vec<_>>(),
            |cycles, (i, direction)| {
                *cycles = cycles
                    .iter_mut()
                    .map(|c| Cycle {
                        key: match direction {
                            'L' => waypoints[c.key].0,
                            _ => waypoints[c.key].1,
                        },
                        start: c.start,
                        period: c.period,
                    })
                    .map(
                        |c| match (c.start, c.period, c.key.chars().nth(2).unwrap()) {
                            (None, None, 'Z') => Cycle {
                                key: c.key,
                                start: Some(i),
                                period: None,
                            },
                            (Some(s), None, 'Z') => Cycle {
                                key: c.key,
                                start: Some(s),
                                period: Some(i - s),
                            },
                            _ => c,
                        },
                    )
                    .collect::<Vec<_>>();
                cycles.iter().any(|n| n.period.is_none()).then_some(
                    cycles
                        .iter()
                        .map(|c| c.start.map(|s| s + 1))
                        .collect::<Vec<_>>(),
                )
            },
        )
        .last()
        .unwrap()
        .iter()
        .map(|v| v.unwrap())
        .collect::<Vec<_>>();

    let result = (1..)
        .find(|n| cycles.iter().all(|c| (n * cycles[0]) % c == 0))
        .unwrap();
    println!("{}", result * cycles[0]);
}
