use std::str::FromStr;

#[derive(Debug)]
struct Rock {
    x: u32,
    y: u32,
    is_round: bool,
}

impl FromStr for Rock {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "O" => Ok(Rock {x: 0,y: 0,is_round: true}),
            "#" => Ok(Rock {x: 0,y: 0,is_round: false}),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn move_until_blocked(rock: &mut Rock, direction: Direction, mut blocked: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let x = rock.x;
    let y = rock.y;
    blocked[x as usize][y as usize] = false;
    match direction {
        Direction::North => {
            for i in (0..=x).rev() {
                if i == 0 && !blocked[0][y as usize]{
                    blocked[0][y as usize] = true;
                    rock.x = 0;
                    break;
                } else if !blocked[i as usize][y as usize] {
                    rock.x = i;
                } else if blocked[i as usize][y as usize] {
                    blocked[rock.x as usize][y as usize] = true;
                    break;
                }
            }
            return blocked;
        },
        _ => return blocked,

    }
}

fn tilt(tilt_to: Direction, mut blocked: Vec<Vec<bool>>, mut round_rocks: Vec<Rock>, cornered_rocks: Vec<Rock>) -> Vec<Vec<bool>>{
    match tilt_to {
        Direction::North => {
            round_rocks.sort_by(|a, b| a.x.cmp(&b.x));
            for rock in round_rocks.iter_mut() {
                // check where blocked in x axis
                blocked = move_until_blocked(rock, tilt_to.clone(), blocked);

            }
            blocked
        },
        _ => blocked,
    }
}

fn solve1(input: &str) -> u32 {
    let lines = input.lines();
    let len = lines.clone().count();
    let mut round_rocks: Vec<Rock> = vec![];
    let mut cornered_rocks: Vec<Rock> = vec![];
    let mut blocked: Vec<Vec<bool>> = vec![vec![false;len];len];

    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c.to_string().parse::<Rock>() {
                Ok(Rock {x: 0, y: 0, is_round: true}) => {
                    round_rocks.push(Rock {x: i as u32, y: j as u32, is_round: true});
                    blocked[i][j] = true;
                },
                Ok(Rock {x: 0, y: 0, is_round: false}) => {
                    cornered_rocks.push(Rock {x: i as u32, y: j as u32, is_round: false});
                    blocked[i][j] = true;
                },
                _ => continue,
            }
        }
    }
    println!("{round_rocks:?}");
    println!("{cornered_rocks:?}");
    println!("{blocked:?}");
    let new_blocked = tilt(Direction::North, blocked, round_rocks, cornered_rocks);
    0
}

fn solve2(_input: &str) -> u128 {
    0
}

pub fn solve() {
    let input = include_str!("input");
    println!("Solution 1: {}", solve1(input));
    println!("Solution 2: {}", solve2(input));
}

#[cfg(test)]

mod tests {
    use crate::fourteen::*;

    const SAMPLE: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

    #[test]
    fn sample_solved1() {
        let res = solve1(SAMPLE);
        assert_eq!(res, 136);
    }

    #[test]
    fn sample_solved2() {
        let res = solve2(SAMPLE);
        assert_eq!(res, 30);
    }

    #[test]
    fn test_move_x1y2_to_x0y2() {
        let mut r = Rock { x: 1, y: 2, is_round: true};
        let mut blocked = vec![vec![false; 10]; 10];
        blocked[0][0] = true;
        blocked[0][5] = true;
        let blocked = move_until_blocked(&mut r, Direction::North, blocked);
        for line in &blocked {
            println!("{line:?}");
        }
        assert_eq!(0, r.x);
        assert_eq!(2, r.y);
        assert_eq!(false, blocked[1][2]);
        assert_eq!(true, blocked[0][2]);
    }

    // Should not move if already blocked
    #[test]
    fn test_move_sample_x1y0_to_x1y0() {
        let mut r = Rock { x: 1, y: 0, is_round: true};
        let mut blocked = vec![vec![false; 10]; 10];
        blocked[0][0] = true;
        blocked[1][0] = true;
        let blocked = move_until_blocked(&mut r, Direction::North, blocked);
        assert_eq!(1, r.x);
        assert_eq!(0, r.y);
        assert_eq!(true, blocked[0][0]);
        assert_eq!(true, blocked[1][0]);
    }
}
