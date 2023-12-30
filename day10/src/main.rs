#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Down,
    Right,
}

fn main() {
    let turnings = vec![
        (('|', Direction::Down), Direction::Down),
        (('|', Direction::Up), Direction::Up),
        (('-', Direction::Left), Direction::Left),
        (('-', Direction::Right), Direction::Right),
        (('L', Direction::Down), Direction::Right),
        (('L', Direction::Left), Direction::Up),
        (('J', Direction::Down), Direction::Left),
        (('J', Direction::Right), Direction::Up),
        (('7', Direction::Right), Direction::Down),
        (('7', Direction::Up), Direction::Left),
        (('F', Direction::Left), Direction::Down),
        (('F', Direction::Up), Direction::Right),
    ]
    .into_iter()
    .collect::<std::collections::HashMap<_, _>>();
    let moves: std::collections::HashMap<Direction, (isize, isize)> = vec![
        (Direction::Left, (-1, 0)),
        (Direction::Up, (0, -1)),
        (Direction::Down, (0, 1)),
        (Direction::Right, (1, 0)),
    ]
    .into_iter()
    .collect();
    let rows = {
        let mut rows = std::io::stdin()
            .lines()
            .map(|l| format!(".{}", l.unwrap()).chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        rows.insert(0, (0..rows[0].len()).map(|_| '.').collect::<Vec<_>>());
        rows.push((0..rows[0].len()).map(|_| '.').collect::<Vec<_>>());
        rows
    };

    let y = rows.iter().position(|row| row.contains(&'S')).unwrap();
    let x = rows[y].iter().position(|c| *c == 'S').unwrap();
    let direction = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .into_iter()
    .find(|d| {
        turnings.contains_key(&(
            rows[(y as isize + moves[d].1) as usize][(x as isize + moves[d].0) as usize],
            *d,
        ))
    })
    .unwrap();

    let steps = std::iter::successors(Some(((x, y), direction)), |((x, y), d)| {
        let x = (*x as isize + moves[d].0) as usize;
        let y = (*y as isize + moves[d].1) as usize;
        (rows[y][x] != 'S').then(|| {
            let d = turnings[&(rows[y][x], *d)];
            ((x, y), d)
        })
    })
    .collect::<std::collections::HashMap<_, _>>();
    println!("{:?}", steps.len() / 2);

    let rows = {
        let s_char = {
            if "F-L".contains(rows[y][x - 1]) {
                if "J|L".contains(rows[y + 1][x]) {
                    '7'
                } else if "-J7".contains(rows[y][x + 1]) {
                    '-'
                } else if "7|F".contains(rows[y - 1][x]) {
                    'J'
                } else {
                    unreachable!()
                }
            } else if "LJ|".contains(rows[y + 1][x]) {
                if "|F7".contains(rows[y - 1][x]) {
                    '|'
                } else if "J7-".contains(rows[y][x + 1]) {
                    'F'
                } else {
                    unreachable!()
                }
            } else {
                'L'
            }
        };

        let mut rows = rows;
        rows[y][x] = s_char;
        rows
    };

    let result = rows
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .fold(
                    (None, false, 0),
                    |(border_start, is_inside, count), (x, ch)| match (
                        steps.contains_key(&(x, y)),
                        border_start,
                        ch,
                    ) {
                        (false, _, _) if is_inside => (border_start, is_inside, count + 1),
                        (false, _, _) => (border_start, is_inside, count),
                        (true, None, '|') => (None, !is_inside, count),
                        (true, None, _) => (Some(*ch), is_inside, count),
                        (true, Some(_), '-') => (border_start, is_inside, count),
                        (true, Some('L'), '7') => (None, !is_inside, count),
                        (true, Some('L'), 'J') => (None, is_inside, count),
                        (true, Some('F'), 'J') => (None, !is_inside, count),
                        (true, Some('F'), '7') => (None, is_inside, count),
                        _ => unreachable!(),
                    },
                )
                .2
        })
        .sum::<usize>();

    println!("{}", result);
}
