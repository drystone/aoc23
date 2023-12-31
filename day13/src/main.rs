fn main() {
    let patterns =
        std::io::stdin()
            .lines()
            .map(|l| l.unwrap())
            .fold(vec![vec![]], |mut patterns, line| {
                if line.is_empty() {
                    patterns.push(vec![]);
                } else {
                    let mut pattern = patterns.pop().unwrap();
                    pattern.push(line);
                    patterns.push(pattern);
                }
                patterns
            });

    let result = patterns
        .iter()
        .fold(0, |acc, pattern| acc + unsmudged(pattern));

    println!("{:?}", result);

    let result = patterns
        .iter()
        .fold(0, |acc, pattern| acc + smudged(pattern));

    println!("{:?}", result);
}

fn horizontal_mirror(pattern: &[String]) -> Vec<usize> {
    (1..pattern.len())
        .filter(|pos| {
            pattern[..*pos]
                .iter()
                .rev()
                .zip(pattern[*pos..].iter())
                .all(|(l1, l2)| l1 == l2)
        })
        .collect()
}

fn vertical_mirror(pattern: &[String]) -> Vec<usize> {
    horizontal_mirror(&transpose(pattern))
}

fn transpose(pattern: &[String]) -> Vec<String> {
    let mut iterators = pattern.iter().map(|s| s.chars()).collect::<Vec<_>>();
    (0..pattern[0].len())
        .map(|_| {
            iterators
                .iter_mut()
                .map(|it| it.next().unwrap())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}

fn unsmudged(pattern: &[String]) -> usize {
    horizontal_mirror(pattern)
        .iter()
        .map(|r| r * 100)
        .chain(vertical_mirror(pattern))
        .next()
        .unwrap()
}

fn smudged(pattern: &[String]) -> usize {
    let unsmudged_score = unsmudged(pattern);
    (0..pattern[0].len())
        .flat_map(move |x| (0..pattern.len()).map(move |y| (x, y)))
        .flat_map(|(x, y)| {
            let pattern = smudge(pattern, x, y);
            horizontal_mirror(&pattern)
                .iter()
                .map(|v| 100 * v)
                .chain(vertical_mirror(&pattern))
                .collect::<Vec<_>>()
        })
        .find(|v| *v != unsmudged_score)
        .unwrap()
}

fn smudge(pattern: &[String], x: usize, y: usize) -> Vec<String> {
    pattern
        .iter()
        .enumerate()
        .map(|(y1, row)| {
            row.chars()
                .enumerate()
                .map(|(x1, ch)| match (x.cmp(&x1), y.cmp(&y1), ch) {
                    (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal, '.') => '#',
                    (std::cmp::Ordering::Equal, std::cmp::Ordering::Equal, '#') => '.',
                    _ => ch,
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}
