use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::{Down, Left, Right, Up};

fn main() {
    let city = {
        let lines = std::io::stdin()
            .lines()
            .map(|l| {
                format!("0{}0", l.unwrap())
                    .chars()
                    .map(|c| c as usize - '0' as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut city = (0..2)
            .map(|_| (0..lines[0].len()).map(|_| 0).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        city.splice(1..1, lines);
        city
    };

    let mut frontier: HashSet<(usize, usize, Direction, usize)> =
        [(1, 1, Right, 3), (1, 1, Down, 3)].into_iter().collect();
    let mut losses: HashMap<(usize, usize, Direction, usize), usize> =
        frontier.iter().cloned().map(|key| (key, 0)).collect();

    while !frontier.is_empty() {
        frontier = frontier.into_iter().fold(
            HashSet::new(),
            |mut frontier, (x, y, direction, balance)| {
                let loss = losses.get(&(x, y, direction, balance)).cloned().unwrap();
                match direction {
                    Up => vec![Up, Left, Right],
                    Down => vec![Down, Left, Right],
                    Left => vec![Up, Down, Left],
                    Right => vec![Up, Down, Right],
                }
                .into_iter()
                .map(|d| match d {
                    Up => (x, y - 1, d),
                    Down => (x, y + 1, d),
                    Left => (x - 1, y, d),
                    Right => (x + 1, y, d),
                })
                .map(|(x, y, d)| {
                    let b = if d == direction { balance - 1 } else { 3 };
                    (x, y, d, b)
                })
                .filter(|(x, y, _, b)| *b > 0 && city[*y][*x] > 0)
                .for_each(|(x, y, d, b)| {
                    let loss = loss + city[y][x];
                    if loss < losses.get(&(x, y, d, b)).cloned().unwrap_or(1_000_000) {
                        losses.insert((x, y, d, b), loss);
                        frontier.insert((x, y, d, b));
                    }
                });
                frontier
            },
        );
    }

    let result = losses
        .iter()
        .filter_map(|((x, y, _, _), loss)| {
            (*x == city[0].len() - 2 && *y == city.len() - 2).then_some(loss)
        })
        .min()
        .unwrap();

    println!("{:?}", result);

    let mut frontier: HashSet<(usize, usize, Direction, usize)> =
        [(1, 1, Right, 11), (1, 1, Down, 11)].into_iter().collect();
    let mut losses: HashMap<(usize, usize, Direction, usize), usize> =
        frontier.iter().cloned().map(|key| (key, 0)).collect();

    while !frontier.is_empty() {
        frontier = frontier.into_iter().fold(
            HashSet::new(),
            |mut frontier, (x, y, direction, balance)| {
                let loss = losses.get(&(x, y, direction, balance)).cloned().unwrap();
                match direction {
                    Up => vec![Up, Left, Right],
                    Down => vec![Down, Left, Right],
                    Left => vec![Up, Down, Left],
                    Right => vec![Up, Down, Right],
                }
                .into_iter()
                .filter(|d| *d == direction || balance <= 7)
                .map(|d| match d {
                    Up => (x, y - 1, d),
                    Down => (x, y + 1, d),
                    Left => (x - 1, y, d),
                    Right => (x + 1, y, d),
                })
                .map(|(x, y, d)| {
                    let b = if d == direction { balance - 1 } else { 10 };
                    (x, y, d, b)
                })
                .filter(|(x, y, _, b)| *b > 0 && city[*y][*x] > 0)
                .for_each(|(x, y, d, b)| {
                    let loss = loss + city[y][x];
                    if loss < losses.get(&(x, y, d, b)).cloned().unwrap_or(1_000_000) {
                        losses.insert((x, y, d, b), loss);
                        frontier.insert((x, y, d, b));
                    }
                });
                frontier
            },
        );
    }

    let result = losses
        .into_iter()
        .filter(|((x, y, _, b), _)| *x == city[0].len() - 2 && *y == city.len() - 2 && *b <= 7)
        .map(|((_, _, _, _), loss)| loss)
        .min()
        .unwrap();

    println!("{:?}", result);
}
