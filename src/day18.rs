type GeneratorType = String;

#[aoc_generator(day18)]
fn generator(input: &str) -> Vec<GeneratorType> {
    input.split('\n').map(|s| s.to_string()).collect()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum M {
    Add,
    Mult,
    Val(u64),
    Paren,
}

impl M {
    fn eval(&self, a: u64, b: u64) -> u64 {
        match self {
            M::Add => a + b,
            M::Mult => a * b,
            _ => unreachable!(),
        }
    }
}

fn math2(input: &[&str]) -> u64 {
    let mut stack = Vec::new();
    let mut pos = 0;
    while pos < input.len() {
        match input[pos] {
            "+" => stack.push(M::Add),
            "*" => stack.push(M::Mult),
            "(" => stack.push(M::Paren),
            ")" => {
                let mut acc = 0;
                let mut a = stack.pop();
                loop {
                    match a.unwrap() {
                        M::Add => {
                            if let Some(M::Val(b)) = stack.pop() {
                                acc += b;
                            } else {
                                panic!("Stack in weird state {:?}", stack)
                            }
                        }
                        M::Mult => {
                            if let Some(M::Val(b)) = stack.pop() {
                                acc *= b;
                            } else {
                                panic!("Stack in weird state {:?}", stack)
                            }
                        }
                        M::Val(b) => acc = b,
                        M::Paren => {
                            if let Some(M::Add) = stack.last() {
                                stack.pop();
                                if let Some(M::Val(b)) = stack.pop() {
                                    acc += b;
                                } else {
                                    panic!("Stack in weird state {:?}", stack)
                                }
                            }
                            stack.push(M::Val(acc));
                            break;
                        }
                    }
                    a = stack.pop();
                }
            }
            c => {
                let mut acc = c.parse().unwrap();
                if let Some(M::Add) = stack.last() {
                    stack.pop();
                    if let Some(M::Val(b)) = stack.pop() {
                        acc += b;
                    } else {
                        panic!("Stack in weird state {:?}", stack)
                    }
                }
                stack.push(M::Val(acc))
            }
        }
        pos += 1;
    }
    stack
        .iter()
        .filter_map(|v| if let M::Val(v) = v { Some(v) } else { None })
        .product()
}

fn math(input: &[&str]) -> (usize, u64) {
    let mut acc = 0;
    let mut cmd = M::Add;
    let mut pos = 0;
    while pos < input.len() {
        match input[pos] {
            "+" => cmd = M::Add,
            "*" => cmd = M::Mult,
            "(" => {
                let (dist, val) = math(&input[(pos + 1)..]);
                pos += dist;
                acc = cmd.eval(acc, val);
            }
            ")" => return (pos + 1, acc),
            c => {
                acc = cmd.eval(acc, c.parse().unwrap());
            }
        }
        pos += 1;
    }
    (pos, acc)
}

fn parse(s: &str) -> impl Iterator<Item = &str> {
    if s.starts_with('(') {
        let (a, mut b) = s.split_at(1);
        let mut res = vec![a];
        while b.starts_with('(') {
            let x = b.split_at(1);
            res.push("(");
            b = x.1;
        }
        res.push(b);
        res.into_iter()
    } else if s.ends_with(')') {
        let (mut a, b) = s.split_at(s.len() - 1);
        let mut res = vec![b];
        while a.ends_with(')') {
            res.push(")");
            let x = a.split_at(a.len() - 1);
            a = x.0;
        }
        res.push(a);
        res.reverse();
        res.into_iter()
    } else {
        vec![s].into_iter()
    }
}

#[aoc(day18, part1)]
fn solve_part1(input: &[GeneratorType]) -> u64 {
    input
        .iter()
        .map(|l| l.split_whitespace().flat_map(parse).collect::<Vec<&str>>())
        .map(|expr| math(expr.as_slice()).1)
        .sum()
}

#[aoc(day18, part2)]
fn solve_part2(input: &[GeneratorType]) -> u64 {
    input
        .iter()
        .map(|l| l.split_whitespace().flat_map(parse).collect::<Vec<&str>>())
        .map(|expr| math2(expr.as_slice()))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(generator(SAMPLE).as_slice()), 13632);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(generator(SAMPLE).as_slice()), 23340);
    }
}
