use std::io::BufRead;

fn main() {
    let lines = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.expect("line"))
        .map(|l| format!(".{l}."))
        .collect::<Vec<_>>();
    let width = lines[0].len();
    let diagram = (0..width)
        .map(|_| ".".to_string())
        .chain(lines.into_iter())
        .chain((0..width).map(|_| ".".to_string()))
        .collect::<String>()
        .chars()
        .collect::<Vec<_>>();
    let result = diagram
        .iter()
        .enumerate()
        .skip(width + 1)
        .fold(0, |result, (i, c)| {
            result
                + c.is_ascii_digit()
                    .then(|| oi(&diagram, width, i))
                    .unwrap_or(0)
        });
    println!("oi: {result}");
    let result = diagram
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == '*')
        .map(|(i, _)| oioi(&diagram, width, i))
        .filter(|parts| parts.len() == 2)
        .map(|parts| parts[0] * parts[1])
        .sum::<usize>();
    println!("oioi: {result}");
}

fn oi(diagram: &[char], width: usize, i: usize) -> usize {
    (diagram[i].is_ascii_digit() && !diagram[i - 1].is_ascii_digit())
        .then(|| {
            (i..)
                .take_while(|i| diagram[*i].is_ascii_digit())
                .flat_map(|i| {
                    (0..=2).flat_map(move |x| {
                        (0..=2).map(move |y| (i, diagram[i + x + y * width - width - 1]))
                    })
                })
                .any(|(_, c)| c == '*')
                .then(|| {
                    (i..)
                        .map(|i| diagram[i])
                        .take_while(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap()
                })
                .unwrap_or(0)
        })
        .unwrap_or(0)
}

fn oioi(diagram: &[char], width: usize, i: usize) -> Vec<usize> {
    (0..=2)
        .flat_map(move |x| (0..=2).map(move |y| i + x + y * width - width - 1))
        .filter(|i| diagram[*i].is_ascii_digit())
        .map(|i| {
            (0..i)
                .map(|j| i - j)
                .take_while(|i| diagram[*i].is_ascii_digit())
                .last()
                .unwrap()
        })
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .map(|i| {
            (i..)
                .map(|i| diagram[i])
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .collect::<Vec<_>>()
}
