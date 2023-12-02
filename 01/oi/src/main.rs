use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let digits = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"]
        .iter()
        .enumerate()
        .map(|(i, s)| (s.to_string(), i as u32))
        .collect::<HashMap<_, _>>();
    let words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let words = words
        .iter()
        .enumerate()
        .map(|(i, s)| (s.to_string(), i as u32))
        .collect::<HashMap<_, _>>();
    let reader = std::io::BufReader::new(std::io::stdin());
    let lines = reader
        .lines()
        .map(|l| l.expect("string"))
        .collect::<Vec<_>>();
    let result = lines.iter().fold(0, |acc, l| acc + oi(l, &digits));
    println!("oi: {}", result);
    let result = lines.iter().fold(0, |acc, l| {
        acc + oi(
            l,
            &digits.clone().into_iter().chain(words.clone()).collect(),
        )
    });
    println!("oioi: {}", result);
}

fn oi(s: &str, tokens: &HashMap<String, u32>) -> u32 {
    let rtokens = tokens
        .iter()
        .map(|(k, v)| (k.chars().rev().collect(), *v))
        .collect::<HashMap<_, _>>();
    oioi(s, tokens) * 10 + oioi(&s.chars().rev().collect::<String>(), &rtokens)
}

fn oioi(s: &str, tokens: &HashMap<String, u32>) -> u32 {
    (0..s.chars().count())
        .map(|i| {
            let s = s.chars().skip(i).collect::<String>();
            tokens
                .iter()
                .map(move |(token, value)| s.starts_with(token).then_some(*value))
        })
        .flatten()
        .filter(|v| v.is_some())
        .take(1)
        .next()
        .unwrap()
        .unwrap()
}
