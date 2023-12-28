fn main() {
    let lines = std::io::stdin()
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<_>>();
    let times = lines[0]
        .split(' ')
        .skip(1)
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<usize>().unwrap());
    let distances = lines[1]
        .split(' ')
        .skip(1)
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<usize>().unwrap());
    let races = times.zip(distances).collect::<Vec<_>>();

    let wins = races.iter().fold(vec![], |mut wins, race| {
        wins.push((1..=race.0).filter(|n| n * (race.0 - n) > race.1).count());
        wins
    });

    println!("{:?}", wins.iter().product::<usize>());

    let time = lines[0]
        .strip_prefix("Time:")
        .unwrap()
        .split(' ')
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let distance = lines[1]
        .strip_prefix("Distance:")
        .unwrap()
        .split(' ')
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let wins = (1..=time).filter(|n| n * (time - n) > distance).count();

    println!("{:?}", wins);
}
