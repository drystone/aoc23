use std::io::BufRead;

fn main() {
    let lines = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();
    let result: usize = lines.iter().map(|l| 1 << score(l) >> 1).sum();
    println!("oi: {result}");

    let mut counts = vec![1; lines.len()];
    lines.iter().enumerate().for_each(|(i, l)| {
        (0..score(l)).for_each(|n| counts[i + n + 1] += counts[i]);
    });
    println!("oioi: {}", counts.iter().sum::<usize>());
}

fn score<'a>(l: &'a str) -> usize {
    let (wins, card) = l.split_once(": ").unwrap().1.split_once(" | ").unwrap();
    let split = |s: &'a str| s.split(' ').filter_map(|n| n.parse::<usize>().ok());
    let wins = split(wins).collect::<std::collections::HashSet<_>>();
    split(card).filter(|n| wins.contains(n)).count()
}
