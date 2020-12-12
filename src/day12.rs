use anyhow::Error;
use std::str::FromStr;

type GeneratorType = Command;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Command {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    Left(i64),
    Right(i64),
    Forward(i64),
}

impl FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, val) = s.split_at(1);
        Ok(match dir {
            "N" => Command::North(val.parse()?),
            "S" => Command::South(val.parse()?),
            "E" => Command::East(val.parse()?),
            "W" => Command::West(val.parse()?),
            "L" => Command::Left(val.parse()?),
            "R" => Command::Right(val.parse()?),
            "F" => Command::Forward(val.parse()?),
            _ => unreachable!("Not allowed by problem"),
        })
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Ship {
    east: i64,
    north: i64,
    heading: i64,
}

impl Ship {
    fn new() -> Self {
        Ship {
            east: 0,
            north: 0,
            heading: 90,
        }
    }

    fn mv(&mut self, cmd: &Command) {
        match cmd {
            Command::North(val) => self.north += val,
            Command::South(val) => self.north -= val,
            Command::East(val) => self.east += val,
            Command::West(val) => self.east -= val,
            Command::Left(val) => {
                let mut tmp = self.heading - val;
                while tmp < 0 {
                    tmp += 360;
                }
                self.heading = tmp;
            }
            Command::Right(val) => {
                let mut tmp = self.heading + val;
                while tmp >= 360 {
                    tmp -= 360;
                }
                self.heading = tmp;
            }
            Command::Forward(val) => {
                // val * cos(self.heading) = North,
                // val * sin(self.heading) = East,
                //let (heading, n_mult, e_mult) = if (0.0..90.0).contains(&self.heading) {
                //    (self.heading, 1.0, 1.0)
                //} else if (90.0..180.0).contains(&self.heading) {
                //    (self.heading - 90.0, -1.0, 1.0)
                //} else if (180.0..270.0).contains(&self.heading) {
                //    (self.heading - 180.0, -1.0, -1.0)
                //} else {
                //    (self.heading - 270.0, 1.0, -1.0)
                //};
                //self.north += n_mult * (val * i64::sin(heading * std::i64::consts::PI / 180.0));
                //self.east += e_mult * (val * i64::cos(heading * std::i64::consts::PI / 180.0));

                match self.heading {
                    0 => self.north += val,
                    90 => self.east += val,
                    180 => self.north -= val,
                    270 => self.east -= val,
                    v => unimplemented!("Shouldn't happen: {}", v),
                }
            }
        }
    }

    fn pos(&self) -> i64 {
        i64::abs(self.east) + i64::abs(self.north)
    }
}

#[derive(Debug)]
struct Waypoint {
    north: i64,
    east: i64,
}

impl Waypoint {
    fn new() -> Self {
        Waypoint { north: 1, east: 10 }
    }

    fn mv(&mut self, ship: &mut Ship, cmd: &Command) {
        match cmd {
            Command::North(val) => self.north += val,
            Command::South(val) => self.north -= val,
            Command::East(val) => self.east += val,
            Command::West(val) => self.east -= val,
            Command::Right(val) => {
                let mut tmp = *val;
                while tmp > 0 {
                    let east = self.north;
                    self.north = -self.east;
                    self.east = east;
                    tmp -= 90;
                }
            }
            Command::Left(val) => {
                let mut tmp = *val;
                while tmp > 0 {
                    let east = -self.north;
                    self.north = self.east;
                    self.east = east;
                    tmp -= 90;
                }
            }
            Command::Forward(val) => {
                ship.north += self.north * val;
                ship.east += self.east * val;
            }
        }
    }
}

#[aoc_generator(day12)]
fn generator(input: &str) -> Vec<GeneratorType> {
    input
        .split('\n')
        .map(|line| {
            line.parse::<Command>()
                .expect("Everything in the file should be parsable")
        })
        .collect()
}

#[aoc(day12, part1)]
fn solve_part1(input: &[GeneratorType]) -> i64 {
    let mut ship = Ship::new();
    input.iter().for_each(|cmd| {
        ship.mv(cmd);
    });
    ship.pos()
}

#[aoc(day12, part2)]
fn solve_part2(input: &[GeneratorType]) -> i64 {
    let mut ship = Ship::new();
    let mut waypoint = Waypoint::new();
    input.iter().for_each(|cmd| {
        waypoint.mv(&mut ship, cmd);
        //println!("cmd: {:?}, ship: {:?}, waypoint: {:?}", cmd, ship, waypoint);
    });
    ship.pos()
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "F10
N3
F7
R90
F11";

    #[test]
    fn test_part1() {
        let res = solve_part1(generator(SAMPLE).as_slice());
        assert_eq!(res, 25);
    }
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(generator(SAMPLE).as_slice()), 286);
    }
}
