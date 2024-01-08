#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::{Down, Left, Right, Up};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Beam {
    x: usize,
    y: usize,
    direction: Direction,
}

fn main() {
    let contraption = {
        let lines = std::io::stdin()
            .lines()
            .map(|l| format!("x{}x", l.unwrap()).chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut contraption = (0..2)
            .map(|_| (0..lines[0].len()).map(|_| 'x').collect::<Vec<_>>())
            .collect::<Vec<_>>();
        contraption.splice(1..1, lines);
        contraption
    };

    let result = energise(&contraption, 0, 1, Right);
    println!("{:?}", result);

    let a = (0..contraption.len())
        .map(|y| energise(&contraption, 0, y, Right))
        .max()
        .unwrap();
    let b = (0..contraption.len())
        .map(|y| energise(&contraption, contraption[0].len() - 1, y, Left))
        .max()
        .unwrap();
    let c = (0..contraption[0].len())
        .map(|x| energise(&contraption, x, 0, Down))
        .max()
        .unwrap();
    let d = (0..contraption.len())
        .map(|x| energise(&contraption, x, contraption.len() - 1, Up))
        .max()
        .unwrap();

    println!("{}", [a, b, c, d].iter().max().unwrap())
}

fn energise(contraption: &[Vec<char>], x: usize, y: usize, direction: Direction) -> usize {
    let mut seen = std::collections::HashSet::new();

    std::iter::successors(Some(vec![Beam { x, y, direction }]), |beams| {
        let Beam { direction, x, y } = beams.iter().cloned().last().unwrap();
        let (x, y) = match direction {
            Up => (x, y - 1),
            Down => (x, y + 1),
            Left => (x - 1, y),
            Right => (x + 1, y),
        };

        let new_beams = match (direction, contraption[y][x]) {
            (_, 'x') => vec![],
            (_, '.') => vec![Beam { direction, x, y }],
            (Up, '|') | (Down, '|') => vec![Beam { direction, x, y }],
            (Left, '-') | (Right, '-') => vec![Beam { direction, x, y }],
            (Left, '|') | (Right, '|') => vec![
                Beam {
                    direction: Up,
                    x,
                    y,
                },
                Beam {
                    direction: Down,
                    x,
                    y,
                },
            ],
            (Up, '-') | (Down, '-') => vec![
                Beam {
                    direction: Left,
                    x,
                    y,
                },
                Beam {
                    direction: Right,
                    x,
                    y,
                },
            ],
            (Up, '\\') => vec![Beam {
                direction: Left,
                x,
                y,
            }],
            (Down, '\\') => vec![Beam {
                direction: Right,
                x,
                y,
            }],
            (Left, '\\') => vec![Beam {
                direction: Up,
                x,
                y,
            }],
            (Right, '\\') => vec![Beam {
                direction: Down,
                x,
                y,
            }],
            (Up, '/') => vec![Beam {
                direction: Right,
                x,
                y,
            }],
            (Down, '/') => vec![Beam {
                direction: Left,
                x,
                y,
            }],
            (Left, '/') => vec![Beam {
                direction: Down,
                x,
                y,
            }],
            (Right, '/') => vec![Beam {
                direction: Up,
                x,
                y,
            }],
            x => {
                println!("{:?}", x);
                unreachable!();
            }
        }
        .into_iter()
        .filter(|beam| !seen.contains(beam))
        .collect::<Vec<_>>();

        new_beams.iter().for_each(|Beam { direction, x, y }| {
            seen.insert(Beam {
                direction: *direction,
                x: *x,
                y: *y,
            });
        });

        let beams = beams[0..beams.len() - 1]
            .iter()
            .chain(new_beams.iter())
            .cloned()
            .collect::<Vec<_>>();

        match beams.len() {
            0 => None,
            _ => Some(beams),
        }
    })
    .for_each(drop);

    seen.iter()
        .map(|Beam { direction: _, x, y }| (x, y))
        .collect::<std::collections::HashSet<_>>()
        .len()
}
