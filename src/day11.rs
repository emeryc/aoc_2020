type GeneratorType = Vec<Element>;

#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    element: Element,
}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Element {
    Empyt,
    Taken,
    Floor,
}

#[aoc_generator(day11)]
fn generator(input: &str) -> Vec<GeneratorType> {
    input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|elem| match elem {
                    'L' => Element::Empyt,
                    '#' => Element::Taken,
                    _ => Element::Floor,
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn run_seats(
    input: &[GeneratorType],
    search: fn(x: usize, y: usize, input: &[GeneratorType]) -> Vec<Element>,
    tolerence: usize,
) -> Vec<GeneratorType> {
    let mut output = input.to_vec();
    for (row_idx, row) in output.iter_mut().enumerate() {
        for (col_idx, elm) in row.iter_mut().enumerate() {
            if col_idx == 9 && row_idx == 0 {
                //println!("{:?}", search(col_idx, row_idx, input));
            }
            match elm {
                Element::Empyt => {
                    if search(col_idx, row_idx, input)
                        .iter()
                        .all(|e| e != &Element::Taken)
                    {
                        *elm = Element::Taken;
                    }
                }
                Element::Taken => {
                    if search(col_idx, row_idx, input)
                        .iter()
                        .filter(|e| *e == &Element::Taken)
                        .count()
                        >= tolerence
                    {
                        *elm = Element::Empyt;
                    }
                }
                Element::Floor => {}
            }
        }
    }
    output
}

fn to_string(map: &[GeneratorType]) -> String {
    let mut layout = Vec::new();

    for y in map.iter() {
        layout.push(
            y.iter()
                .map(|elem| match elem {
                    Element::Empyt => "L",
                    Element::Taken => "#",
                    Element::Floor => ".",
                })
                .collect::<Vec<_>>()
                .join(""),
        );
    }

    layout.join("\n")
}

#[aoc(day11, part1)]
fn solve_part1(input: &[GeneratorType]) -> usize {
    let mut last = input.to_vec();
    let mut count = 0;
    loop {
        let this = run_seats(
            last.as_slice(),
            |x, y, input| {
                let x = x as isize;
                let y = y as isize;
                vec![
                    (x, y + 1),
                    (x + 1, y),
                    (x - 1, y),
                    (x, y - 1),
                    (x + 1, y - 1),
                    (x - 1, y + 1),
                    (x + 1, y + 1),
                    (x - 1, y - 1),
                ]
                .iter()
                .filter(|(x, y)| *x >= 0 && *y >= 0)
                .map(|(x, y)| (*x as usize, *y as usize))
                .filter_map(|(x, y)| input.get(y).and_then(|y| y.get(x)))
                .cloned()
                .collect::<Vec<_>>()
            },
            4,
        );
        if this == last {
            break last
                .iter()
                .flatten()
                .filter(|elm| **elm == Element::Taken)
                .count();
        }
        //println!("iteration {}\n{}\n", count, to_string(last.as_slice()));
        count += 1;
        last = this;
    }
}

#[aoc(day11, part2)]
fn solve_part2(input: &[GeneratorType]) -> usize {
    let mut last = input.to_vec();
    let mut count = 0;
    loop {
        let this = run_seats(
            last.as_slice(),
            |x, y, input| {
                let mut output: Vec<Element> = Vec::new();

                let mut count = 1;
                loop {
                    match input.get(y).and_then(|y| y.get(x + count)) {
                        Some(elm) if *elm != Element::Floor => {
                            output.push(*elm);
                            break;
                        }
                        None => break,
                        _ => {}
                    }
                    count += 1;
                }
                let mut count = 1;
                loop {
                    match input.get(y + count).and_then(|y| y.get(x)) {
                        Some(elm) if *elm != Element::Floor => {
                            output.push(*elm);
                            break;
                        }
                        None => break,
                        _ => {}
                    }
                    count += 1;
                }
                let mut count = 1;
                loop {
                    match input.get(y + count).and_then(|y| y.get(x + count)) {
                        Some(elm) if *elm != Element::Floor => {
                            output.push(*elm);
                            break;
                        }
                        None => break,
                        _ => {}
                    }
                    count += 1;
                }
                let mut count: usize = 1;
                while y as isize - count as isize >= 0 {
                    match input.get(y - count).and_then(|y| y.get(x + count)) {
                        Some(elm) if *elm != Element::Floor => {
                            output.push(*elm);
                            break;
                        }
                        None => break,
                        _ => {}
                    }
                    count += 1;
                }
                let mut count: usize = 1;
                while y as isize - count as isize >= 0 {
                    match input.get(y - count).and_then(|y| y.get(x)) {
                        Some(elm) if *elm != Element::Floor => {
                            output.push(*elm);
                            break;
                        }
                        None => break,
                        _ => {}
                    }
                    count += 1;
                }
                let mut count: usize = 1;
                while y as isize - count as isize >= 0 && x as isize - count as isize >= 0 {
                    match input.get(y - count).and_then(|y| y.get(x - count)) {
                        Some(elm) if *elm != Element::Floor => {
                            output.push(*elm);
                            break;
                        }
                        None => break,
                        _ => {}
                    }
                    count += 1;
                }
                let mut count: usize = 1;
                while x as isize - count as isize >= 0 {
                    match input.get(y + count).and_then(|y| y.get(x - count)) {
                        Some(elm) if *elm != Element::Floor => {
                            output.push(*elm);
                            break;
                        }
                        None => break,
                        _ => {}
                    }
                    count += 1;
                }
                let mut count: usize = 1;
                while x as isize - count as isize >= 0 {
                    match input.get(y).and_then(|y| y.get(x - count)) {
                        Some(elm) if *elm != Element::Floor => {
                            output.push(*elm);
                            break;
                        }
                        None => break,
                        _ => {}
                    }
                    count += 1;
                }
                output
            },
            5,
        );
        if this == last {
            break last
                .iter()
                .flatten()
                .filter(|elm| **elm == Element::Taken)
                .count();
        }
        //println!("iteration {}\n{}\n", count, to_string(last.as_slice()));
        count += 1;
        last = this;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(generator(SAMPLE).as_slice()), 37);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(generator(SAMPLE).as_slice()), 26);
    }
}
