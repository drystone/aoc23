use std::collections::HashMap;

fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();
    let hands = lines
        .iter()
        .map(|l| {
            let mut fields = l.split(' ');
            (
                translate_hand(fields.next().unwrap(), false),
                fields.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut scored_hands = hands
        .iter()
        .map(|(hand, bet)| (format!("{}{}", score_hand(hand), hand), *bet))
        .collect::<Vec<_>>();

    scored_hands.sort();

    let result = scored_hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + (i + 1) * bid);

    println!("{:?}", result);

    let hands = lines
        .iter()
        .map(|l| {
            let mut fields = l.split(' ');
            (
                translate_hand(fields.next().unwrap(), true),
                fields.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut scored_hands = hands
        .iter()
        .map(|(hand, bid)| {
            let score = wildcard_jack_permutations(hand)
                .iter()
                .map(|hand| score_hand(hand))
                .max()
                .unwrap();
            (format!("{}{}", score, hand), *bid)
        })
        .collect::<Vec<_>>();

    scored_hands.sort();

    let result = scored_hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, bid))| acc + (i + 1) * bid);

    println!("{:?}", result);
}

fn score_hand(hand: &str) -> char {
    let mut counts = hand
        .chars()
        .fold(HashMap::new(), |mut counts: HashMap<char, usize>, card| {
            *counts.entry(card).or_insert(0) += 1;
            counts
        })
        .into_values()
        .collect::<Vec<_>>();

    counts.sort();
    match counts[..] {
        [5] => 'z',
        [1, 4] => 'y',
        [2, 3] => 'x',
        [1, 1, 3] => 'w',
        [1, 2, 2] => 'v',
        [1, 1, 1, 2] => 'u',
        _ => 't',
    }
}

fn translate_hand(hand: &str, wildcard_jack: bool) -> String {
    hand.chars()
        .map(|c| match (c, wildcard_jack) {
            ('A', _) => 'e',
            ('K', _) => 'd',
            ('Q', _) => 'c',
            ('J', false) => 'b',
            ('J', true) => '1',
            ('T', _) => 'a',
            _ => c,
        })
        .collect::<String>()
}

fn wildcard_jack_permutations(cards: &str) -> Vec<String> {
    let permutations = cards.chars().next().map(|c| match c {
        '1' => vec!['2', '3', '4', '5', '6', '7', '8', '9', 'a', 'c', 'd', 'e'],
        _ => vec![c],
    });
    permutations
        .map(|permutations| {
            permutations
                .into_iter()
                .flat_map(|c| {
                    wildcard_jack_permutations(&cards[1..])
                        .into_iter()
                        .map(|s| format!("{}{}", c, s))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| vec!["".to_string()])
}
