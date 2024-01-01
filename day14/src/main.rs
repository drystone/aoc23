fn main() {
    let platform = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let result = weight(&settle(&platform));

    println!("{}", result);

    let mut seen = std::collections::HashMap::<Vec<Vec<char>>, usize>::new();

    let platforms = std::iter::successors(Some(platform), |platform| {
        (!seen.contains_key(platform)).then(|| {
            seen.insert(platform.clone(), seen.len());
            (0..4).fold(platform.clone(), |platform, _| rotate(&settle(&platform)))
        })
    })
    .collect::<Vec<_>>();

    let cycle_len = seen.len() - seen[platforms.iter().last().unwrap()];
    let spins_remaining = 1_000_000_000 - platforms.len();
    let cycle_remaining = spins_remaining % cycle_len;

    let result = weight(&platforms[platforms.len() - cycle_len + cycle_remaining]);

    println!("{}", result);
}

fn weight(platform: &[Vec<char>]) -> usize {
    platform
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .map(|c| match c {
                    'O' => platform.len() - i,
                    _ => 0,
                })
                .sum::<usize>()
        })
        .sum()
}

fn settle(platform: &[Vec<char>]) -> Vec<Vec<char>> {
    std::iter::successors(Some(platform.to_vec()), |platform| {
        let new_platform = roll(platform);
        (*platform != new_platform).then_some(new_platform)
    })
    .last()
    .unwrap()
}

fn roll(platform: &[Vec<char>]) -> Vec<Vec<char>> {
    platform[1..]
        .iter()
        .fold(vec![platform[0].clone()], |mut new_platform, uphill| {
            let downhill = new_platform.pop().unwrap();
            let (downhill, uphill): (Vec<_>, Vec<_>) = downhill
                .iter()
                .zip(uphill.iter())
                .map(|(d, u)| match (d, u) {
                    ('.', 'O') => ('O', '.'),
                    (d, u) => (*d, *u),
                })
                .unzip();
            new_platform.push(downhill);
            new_platform.push(uphill);
            new_platform
        })
}

fn rotate(matrix: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut iters = matrix.iter().map(|r| r.iter()).collect::<Vec<_>>();
    (0..matrix[0].len()).fold(vec![], |mut result, _| {
        result.push(
            iters
                .iter_mut()
                .rev()
                .map(|it| *it.next().unwrap())
                .collect::<Vec<_>>(),
        );
        result
    })
}
