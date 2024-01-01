#[derive(Clone)]
struct Lens {
    name: String,
    focal_length: usize,
}

fn main() {
    let steps = std::io::stdin()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>()[0]
        .split(',')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    let result = steps.iter().fold(0, |acc, step| acc + hash(step));

    println!("{}", result);

    let result: usize = steps
        .iter()
        .fold(
            (0..256).map(|_| vec![]).collect::<Vec<Vec<Lens>>>(),
            |mut boxes, step| {
                if let Some((name, focal_length)) = step.split_once('=') {
                    if let Some(pos) = boxes[hash(name)].iter().position(|lens| lens.name == name) {
                        boxes[hash(name)][pos].focal_length = focal_length.parse().unwrap();
                    } else {
                        boxes[hash(name)].push(Lens {
                            name: name.to_string(),
                            focal_length: focal_length.parse().unwrap(),
                        })
                    }
                } else if let Some((name, _)) = step.split_once('-') {
                    boxes[hash(name)] = boxes[hash(name)]
                        .iter()
                        .filter_map(|l| (l.name != name).then_some(l.clone()))
                        .collect();
                }
                boxes
            },
        )
        .iter()
        .enumerate()
        .map(|(box_num, boxx)| {
            boxx.iter()
                .enumerate()
                .map(|(slot_num, lens)| (box_num + 1) * (slot_num + 1) * lens.focal_length)
                .sum::<usize>()
        })
        .sum();

    println!("{}", result);
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |hash, ch| ((hash + (ch as usize)) * 17) & 0xff)
}
