fn main() {
    let galaxy = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let empty_rows = galaxy
        .iter()
        .enumerate()
        .fold(vec![], |mut empties, (i, row)| {
            if !row.contains(&'#') {
                empties.push(i);
            }
            empties
        });
    let empty_columns = galaxy
        .iter()
        .fold(
            (0..galaxy[0].len()).map(|_| None).collect::<Vec<_>>(),
            |mut markers, row| {
                row.iter().enumerate().for_each(|(i, ch)| {
                    if *ch == '#' {
                        markers[i] = Some(());
                    }
                });
                markers
            },
        )
        .iter()
        .enumerate()
        .filter(|(_, g)| g.is_none())
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    let universes = galaxy
        .iter()
        .enumerate()
        .fold(vec![], |universes, (y, row)| {
            row.iter()
                .enumerate()
                .fold(universes, |mut universes, (x, ch)| {
                    if *ch == '#' {
                        universes.push((x, y));
                    }
                    universes
                })
        });

    let result = (0..universes.len()).fold(0, |total_distance, f| {
        (f..universes.len()).fold(total_distance, |total_distance, t| {
            total_distance + distance(universes[f], universes[t], &empty_rows, &empty_columns, 2)
        })
    });
    println!("{:?}", result);

    let result = (0..universes.len()).fold(0, |total_distance, f| {
        (f..universes.len()).fold(total_distance, |total_distance, t| {
            total_distance
                + distance(
                    universes[f],
                    universes[t],
                    &empty_rows,
                    &empty_columns,
                    1_000_000,
                )
        })
    });
    println!("{:?}", result);
}

fn distance(
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
    empty_rows: &[usize],
    empty_columns: &[usize],
    expansion: usize,
) -> usize {
    let (x1, x2) = match x1.cmp(&x2) {
        std::cmp::Ordering::Greater => (x2, x1),
        _ => (x1, x2),
    };
    let (y1, y2) = match y1.cmp(&y2) {
        std::cmp::Ordering::Greater => (y2, y1),
        _ => (y1, y2),
    };
    x2 - x1 + y2 - y1
        + empty_rows.iter().filter(|y| *y > &y1 && *y < &y2).count() * (expansion - 1)
        + empty_columns
            .iter()
            .filter(|x| *x > &x1 && *x < &x2)
            .count()
            * (expansion - 1)
}
