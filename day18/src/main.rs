#[derive(Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug)]
struct Trench {
    direction: Direction,
    length: usize,
    colour: String,
}

fn main() {
    let trenches = {
        std::io::stdin()
            .lines()
            .flatten()
            .map(|l| {
                let mut l = l.split(' ');
                Trench {
                    direction: match l.next().unwrap() {
                        "U" => Direction::U,
                        "D" => Direction::D,
                        "L" => Direction::L,
                        "R" => Direction::R,
                        _ => unreachable!(),
                    },
                    length: l.next().unwrap().parse().unwrap(),
                    colour: l.next().unwrap().to_string(),
                }
            })
            .collect::<Vec<_>>()
    };

    println!("{}", solve(&trenches));
    println!(
        "{}",
        solve(
            &trenches
                .iter()
                .map(|trench| {
                    let length_and_direction =
                        i64::from_str_radix(&trench.colour[2..trench.colour.len() - 1], 16)
                            .unwrap();
                    let length = (length_and_direction >> 4) as usize;
                    match length_and_direction & 0x0f {
                0 /* R */ => Trench {
                        direction: Direction::R,
                        length,
                        colour: String::new(),
                    },
                1 /* D */ => Trench {
                        direction: Direction::D,
                        length,
                        colour: String::new(),
                    },
                2 /* L */ => Trench {
                        direction: Direction::L,
                        length,
                        colour: String::new(),
                    },
                3 /* U */ => Trench {
                        direction: Direction::U,
                        length,
                        colour: String::new(),
                    },
                _ => unreachable!(),
                }
                })
                .collect::<Vec<_>>()
        )
    );
}

fn solve(trenches: &[Trench]) -> i64 {
    let trench = trenches
        .iter()
        .map(|trench| trench.length as i64)
        .sum::<i64>();

    let lines = {
        let mut lines = trenches
            .iter()
            .scan((0, 0), |coords, trench| {
                let length = trench.length as i64;
                match trench.direction {
                    Direction::R => {
                        coords.0 += length;
                        Some(None)
                    }
                    Direction::D => {
                        coords.1 -= length;
                        Some(Some((coords.0, coords.1, coords.1 + length)))
                    }
                    Direction::L => {
                        coords.0 -= length;
                        Some(None)
                    }
                    Direction::U => {
                        coords.1 += length;
                        Some(Some((coords.0, coords.1 - length, coords.1)))
                    }
                }
            })
            .flatten()
            .collect::<Vec<_>>();

        lines.sort();

        lines
    };

    let slices = {
        let mut ys = lines
            .iter()
            .map(|(_, y, _)| *y)
            .chain(lines.iter().map(|(_, _, y)| *y))
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        ys.sort();

        ys.iter()
            .zip(ys[1..].iter())
            .map(|(y1, y2)| (*y1, *y2))
            .collect::<Vec<_>>()
    };

    let area = slices
        .iter()
        .map(|(slice_bottom, slice_top)| {
            let lines = lines
                .iter()
                .filter(|(_, line_bottom, line_top)| {
                    line_top > slice_bottom && line_bottom < slice_top
                })
                .collect::<Vec<_>>();
            lines
                .chunks(2)
                .map(|pair| (slice_top - slice_bottom) * (pair[1].0 - pair[0].0))
                .sum::<i64>()
        })
        .sum::<i64>();

    area + trench / 2 + 1
}
