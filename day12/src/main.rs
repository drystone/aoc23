#[derive(Debug)]
struct Puzzle {
    input: Vec<char>,
    solution: Vec<usize>,
}

fn main() {
    let puzzles = {
        std::io::stdin()
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                let (input, solution) = l.split_once(' ').unwrap();
                Puzzle {
                    input: input.chars().collect(),
                    solution: solution.split(',').map(|v| v.parse().unwrap()).collect(),
                }
            })
            .collect::<Vec<_>>()
    };

    let result = puzzles.iter().fold(0, |count, Puzzle { input, solution }| {
        count
            + permutations(
                input,
                solution,
                false,
                &mut std::collections::HashMap::new(),
            )
            .unwrap_or(0)
    });

    println!("{:?}", result);

    let puzzles = puzzles
        .into_iter()
        .map(|Puzzle { input, solution }| {
            let input = (1..5)
                .fold(input.clone(), |mut acc, _| {
                    acc.push('?');
                    acc.extend_from_slice(&input);
                    acc
                })
                .to_vec();
            let solution = solution
                .iter()
                .cycle()
                .copied()
                .take(solution.len() * 5)
                .collect();
            Puzzle { input, solution }
        })
        .collect::<Vec<_>>();

    let result = puzzles.iter().fold(0, |count, Puzzle { input, solution }| {
        count
            + permutations(
                input,
                solution,
                false,
                &mut std::collections::HashMap::new(),
            )
            .unwrap_or(0)
    });

    println!("{:?}", result);
}

fn permutations(
    input: &[char],
    solution: &[usize],
    in_springs: bool,
    seen: &mut std::collections::HashMap<(Vec<char>, Vec<usize>, bool), Option<usize>>,
) -> Option<usize> {
    let key = (input.to_vec(), solution.to_vec(), in_springs);
    if seen.contains_key(&key) {
        seen[&key]
    } else if input.len() + 1 < solution.iter().sum::<usize>() + solution.len() {
        seen.insert(key, None);
        None
    } else {
        let result = match (
            input.iter().next(),
            in_springs,
            solution.iter().next().copied(),
        ) {
            (Some('.'), false, None) => (!input.iter().any(|c| *c == '#')).then_some(1),
            (Some('.'), false, _) => permutations(&input[1..], solution, false, seen),
            (Some('.'), true, Some(0)) => permutations(&input[1..], &solution[1..], false, seen),
            (Some('#'), false, Some(n))
                if input.len() >= n && !input.iter().take(n).any(|c| *c == '.') =>
            {
                permutations(&input[n..], &append(0, &solution[1..]), true, seen)
            }
            (Some('?'), _, _) => {
                let p1 = permutations(&append('.', &input[1..]), solution, in_springs, seen);
                let p2 = permutations(&append('#', &input[1..]), solution, in_springs, seen);
                match (p1, p2) {
                    (None, None) => None,
                    (Some(p1), None) => Some(p1),
                    (None, Some(p2)) => Some(p2),
                    (Some(p1), Some(p2)) => Some(p1 + p2),
                }
            }
            (None, true, Some(0)) if solution.len() == 1 => Some(1),
            (None, false, None) => Some(1),
            _ => None,
        };
        seen.insert(key, result);
        result
    }
}

fn append<T>(head: T, tail: &[T]) -> Vec<T>
where
    T: Clone,
{
    let mut result = vec![head];
    result.extend_from_slice(tail);
    result
}
