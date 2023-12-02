use std::io::BufRead;

fn main() {
    let lines = std::io::BufReader::new(std::io::stdin())
        .lines()
        .map(|l| l.expect("line"))
        .collect::<Vec<_>>();
    let result: usize = lines.iter().fold(0, |result, line| {
        let (game_id, draws) = line.split_once(": ").expect("first split");
        result
            + if [("red", 12), ("green", 13), ("blue", 14)]
                .into_iter()
                .all(|(c, n)| oi(draws, c) <= n)
            {
                game_id
                    .trim_start_matches("Game ")
                    .parse()
                    .expect("integer id")
            } else {
                0
            }
    });
    println!("oi: {result}");

    let result: usize = lines.iter().fold(0, |result, line| {
        let draws = line.split(": ").nth(1).expect("first split");
        result
            + ["red", "green", "blue"]
                .into_iter()
                .map(|c| oi(draws, c))
                .product::<usize>()
    });
    println!("oioi: {result}");
}

fn oi(draws: &str, colour: &str) -> usize {
    draws
        .split("; ")
        .flat_map(|draw| {
            draw.split(", ").map(|count| {
                let (n, c) = count.split_once(' ').expect("count");
                (c == colour)
                    .then_some(n)
                    .map(|n| n.parse().expect("usize count"))
                    .unwrap_or(0)
            })
        })
        .max()
        .expect("counts")
}
