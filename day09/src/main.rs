fn main() {
    let puzzles = std::io::stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .split(' ')
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let result = puzzles.iter().map(|puzzle| solve(puzzle)).sum::<isize>();

    println!("{:?}", result);

    let result = puzzles.iter().map(|puzzle| solve1(puzzle)).sum::<isize>();

    println!("{:?}", result);
}

fn solve(puzzle: &[isize]) -> isize {
    history(puzzle.to_vec())
        .iter()
        .rev()
        .fold(0, |acc, epoch| epoch.iter().last().unwrap() + acc)
}

fn solve1(puzzle: &[isize]) -> isize {
    history(puzzle.to_vec())
        .iter()
        .rev()
        .fold(0, |acc, epoch| epoch.iter().next().unwrap() - acc)
}

fn history(puzzle: Vec<isize>) -> Vec<Vec<isize>> {
    std::iter::successors(Some(puzzle), |epoch| {
        let epoch = epoch[1..]
            .iter()
            .zip(epoch.iter())
            .map(|(a, b)| a - b)
            .collect::<Vec<_>>();
        epoch.iter().any(|v| *v != 0).then_some(epoch.clone())
    })
    .collect::<Vec<_>>()
}
