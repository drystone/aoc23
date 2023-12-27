use std::cmp;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

#[derive(Debug, Copy, Clone)]
struct Filter {
    start: usize,
    end: usize,
    new_start: usize,
}

fn main() {
    let mut input = std::io::stdin().lines();
    let seeds = input
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let (map, _) = input.map(|l| l.unwrap()).filter(|l| !l.is_empty()).fold(
        (
            HashMap::<String, (String, Vec<(usize, usize, usize)>)>::new(),
            (String::new(), String::new()),
        ),
        |(mut map, (from, to)), l| match l.split_once(" map:") {
            Some((l, _)) => {
                let (from, to) = l
                    .split_once("-to-")
                    .map(|(from, to)| (from.to_string(), to.to_string()))
                    .unwrap();
                map.insert(from.to_string(), (to.to_string(), vec![]));
                (map, (from, to))
            }
            None => {
                let parts = l
                    .split(' ')
                    .flat_map(|s| s.parse::<usize>())
                    .collect::<Vec<_>>();
                map.entry(from.clone())
                    .and_modify(|e| e.1.push((parts[0], parts[1], parts[2])));
                (map, (from, to))
            }
        },
    );

    let result = seeds
        .iter()
        .map(|n| {
            std::iter::successors(Some(("seed".to_owned(), *n)), |(from, n)| {
                map.get(from).map(|(to, ranges)| {
                    ranges
                        .iter()
                        .find(|(_, src, size)| n >= src && *n < src + size)
                        .map(|(dest, src, _)| (to.to_string(), n + dest - src))
                        .unwrap_or_else(|| (to.to_string(), *n))
                })
            })
            .last()
            .unwrap()
        })
        .map(|(_, n)| n)
        .min()
        .unwrap();
    println!("{:?}", result);

    let ranges = seeds
        .chunks(2)
        .map(|r| Range {
            start: r[0],
            end: r[0] + r[1],
        })
        .collect::<Vec<_>>();
    let result = std::iter::successors(Some(("seed".to_string(), ranges)), |(from, ranges)| {
        map.get(from).map(|(to, filters)| {
            let (mut unfiltered, mut filtered) = filters
                .iter()
                .map(|(n, s, c)| Filter {
                    new_start: *n,
                    start: *s,
                    end: s + c,
                })
                .fold(
                    (ranges.clone(), vec![]),
                    |(unfiltered, mut filtered), filter| {
                        let (u, mut f) = filter_ranges(&unfiltered, &filter);
                        filtered.append(&mut f);

                        (u, filtered)
                    },
                );

            filtered.append(&mut unfiltered);

            (to.to_string(), filtered)
        })
    })
    .last()
    .map(|(_, ranges)| ranges.into_iter().map(|r| r.start).collect::<Vec<_>>())
    .map(|mut locations| {
        locations.sort();
        locations[0]
    })
    .unwrap();

    println!("{:?}", result);
}

fn filter_ranges(ranges: &[Range], filter: &Filter) -> (Vec<Range>, Vec<Range>) {
    ranges
        .iter()
        .fold((vec![], vec![]), |(mut unfiltered, mut filtered), range| {
            let (before, overlap, after) = filter_range(*range, *filter);

            unfiltered.append(&mut before.map(|x| vec![x]).unwrap_or(vec![]));
            unfiltered.append(&mut after.map(|x| vec![x]).unwrap_or(vec![]));
            filtered.append(&mut overlap.map(|x| vec![x]).unwrap_or(vec![]));

            (unfiltered, filtered)
        })
}

fn filter_range(range: Range, filter: Filter) -> (Option<Range>, Option<Range>, Option<Range>) {
    (
        (range.start < filter.start).then(|| Range {
            start: range.start,
            end: cmp::min(filter.start, range.end),
        }),
        (range.start < filter.end && range.end > filter.start)
            .then(|| Range {
                start: cmp::max(filter.start, range.start),
                end: cmp::min(range.end, filter.end),
            })
            .map(|Range { start, end }| Range {
                start: start + filter.new_start - filter.start,
                end: end + filter.new_start - filter.start,
            }),
        (range.end > filter.end).then(|| Range {
            start: cmp::max(filter.end, range.start),
            end: range.end,
        }),
    )
}
