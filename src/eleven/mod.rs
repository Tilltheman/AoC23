use std::collections::HashSet;

fn expand(mut universe: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rows_to_insert_to: Vec<usize> = vec![];
    let mut cols_to_insert_not_to: Vec<usize> = vec![];
    let rows = universe.len();
    let cols = universe[0].len();
    for (r, row) in universe.iter().enumerate() {
        let mut found_hashes = false;
        for elem in row.iter() {
            if elem == &'#' {
                found_hashes = true;
            }
        }
        if !found_hashes {
            rows_to_insert_to.push(r);
        }
    }
    for row in &universe[..] {
        for (i, (j, _col)) in row.iter().zip(row.iter().rev()).enumerate() {
            if j == &'#' {
                cols_to_insert_not_to.push(i);
            }
        }
    }
    let mut cols_to_insert_to: Vec<usize> = vec![];
    for i in (0..cols).rev() {
        if ! cols_to_insert_not_to.contains(&i) {
            cols_to_insert_to.push(i);
        }
    }
    // insert cols
    let len_cols = cols_to_insert_to.len();
    for col in cols_to_insert_to {
        for i in 0..rows {
            universe[i].insert(col, '.');
        }
    }

    for row in rows_to_insert_to.iter().rev() {
        universe.insert(*row, vec!['.';cols+len_cols]);
    }
    universe
}

#[derive(Debug)]
struct Universe {
    insertions_x: Vec<i128>,
    insertions_y: Vec<i128>,
    universe: Vec<Vec<char>>,
}

impl Universe {
    fn find_insertion_points_y(&mut self) {
        for (r, row) in self.universe.iter().enumerate() {
            let mut found_hashes = false;
            for elem in row.iter() {
                if elem == &'#' {
                    found_hashes = true;
                }
            }
            if !found_hashes {
                self.insertions_x.push(r as i128);
            }
        }
    }
    fn find_insertion_points_x(&mut self) {
        let cols = self.universe[0].len();
        let mut cols_to_insert_not_to: Vec<usize> = vec![];
        for row in &self.universe[..] {
            for (i, (j, _col)) in row.iter().zip(row.iter()).enumerate() {
                if j == &'#' {
                    cols_to_insert_not_to.push(i);
                }
            }
        }
        for i in 0..cols {
            if ! cols_to_insert_not_to.contains(&i) {
                self.insertions_y.push(i as i128);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Galaxy {
    id: i32,
    x: i128,
    y: i128,
    insertions_x: Vec<i128>,
    insertions_y: Vec<i128>,
    multiplier: i128,
}

impl Galaxy {
    fn shortest_path_to(&self, other: &Galaxy) -> i128{
        let x1 = self.x;
        let y1 = self.y;
        let x2 = other.x;
        let y2 = other.y;
        let mut insertions_between_x = 0;
        let mut insertions_between_y = 0;
        if x1 < x2 {
            for x in x1..=x2 {
                if other.insertions_x.contains(&x) {
                    insertions_between_x += 1;
                }
            }
        } else {
            for x in x2..=x1 {
                if self.insertions_x.contains(&x) {
                    insertions_between_x += 1;
                }
            }
        }
        if y1 < y2 {
            for y in y1..=y2 {
                if other.insertions_y.contains(&y) {
                    insertions_between_y += 1;
                }
            }
        } else {
            for y in y2..=y1 {
                if self.insertions_y.contains(&y) {
                    insertions_between_y += 1;
                }
            }
        }
        let count = (x1 - x2).abs() + insertions_between_x  * self.multiplier + (y1 -y2).abs() + insertions_between_y * self.multiplier;
        count
    }
}

fn solve1(input: &str) -> i128 {
    let lines = input.lines();
    let mut sum = 0;
    let amount_rows = lines.clone().count();
    let amount_cols = lines.clone().nth(0).unwrap().chars().count();
    let mut universe: Vec<Vec<char>> = vec![vec!['0'; amount_cols]; amount_rows];
    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            universe[i][j] = c;
        }
    }
    // expand universe
    let universe = expand(universe.clone());
    let mut galaxies: Vec<Galaxy> = vec![];
    let mut id = 1;
    for (i, line) in universe.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if c == &'#' {
                galaxies.push(Galaxy {id: id, x: i as i128, y: j as i128, insertions_x: vec![], insertions_y: vec![], multiplier: 1 });
                id += 1;
            }
        }
    }
    let mut hashset = HashSet::new();
    for (i, el1) in galaxies.iter().enumerate() {
        for (_j, el2) in galaxies[i+1..].iter().enumerate() {
            hashset.insert((el1, el2));
        }
    }
    for element in hashset.iter() {
        sum += element.0.shortest_path_to(element.1);
    }
    sum
}

fn solve2(input: &str, mult: i128) -> i128 {
    let lines = input.lines();
    let mut sum = 0;
    let amount_rows = lines.clone().count();
    let amount_cols = lines.clone().nth(0).unwrap().chars().count();
    let mut universe2: Vec<Vec<char>> = vec![vec!['0'; amount_cols]; amount_rows];
    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            universe2[i][j] = c;
        }
    }
    // create universe
    let mut universe = Universe {
            insertions_x: vec![],
            insertions_y: vec![],
            universe: universe2.clone()
    };
    // find insertion points
    universe.find_insertion_points_x();
    universe.find_insertion_points_y();

    let mut galaxies: Vec<Galaxy> = vec![];
    let mut id = 1;
    for (i, line) in universe2.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if c == &'#' {
                let ins_x: Vec<i128> = universe
                    .insertions_x
                    .iter()
                    .filter_map(|x| if *x < i as i128 { Some(*x) } else { None })
                    .collect();
                let ins_y: Vec<i128> = universe
                    .insertions_y
                    .iter()
                    .filter_map(|y| if *y < j as i128 { Some(*y) } else { None })
                    .collect();
                galaxies.push(Galaxy {id: id, x: i as i128, y: j as i128, insertions_x: ins_x.clone(), insertions_y: ins_y.clone(), multiplier: mult });
                id += 1;
            }
        }
    }
    let mut hashset = HashSet::new();
    for (i, el1) in galaxies.iter().enumerate() {
        for (_j, el2) in galaxies[i+1..].iter().enumerate() {
            hashset.insert((el1, el2));
        }
    }
    for element in hashset.iter() {
        sum += element.0.shortest_path_to(element.1);
    }
    sum
}

pub fn solve() {
    let input = include_str!("input");
    println!("Solution 1: {}", solve1(input));
    println!("Solution 2: {}", solve2(input, 999999));
}

#[cfg(test)]
mod tests {
    use crate::eleven::*;

    const SAMPLE: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    const SAMPLE_EXPANDED: &str = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";


    #[test]
    fn one_solved() {
        let res = solve1(SAMPLE);
        assert_eq!(res, 374);
    }

    #[test]
    fn two_solved_10() {
        let res = solve2(SAMPLE, 9);
        assert_eq!(res, 1030);
    }
    #[test]
    fn two_solved_100() {
        let res = solve2(SAMPLE, 99);
        assert_eq!(res, 8410);
    }

    #[test]
    fn test_expansion_works() {
        let lines = SAMPLE.lines();
        let amount_rows = lines.clone().count();
        let amount_cols = lines.clone().nth(0).unwrap().chars().count();
        let mut universe: Vec<Vec<char>> = vec![vec!['0'; amount_cols]; amount_rows];
        for (i, line) in lines.enumerate() {
            for (j, c) in line.chars().enumerate() {
                universe[i][j] = c;
            }
        }
        // expand universe
        let universe = expand(universe);
        let lines = SAMPLE_EXPANDED.lines();
        let amount_rows = lines.clone().count();
        let amount_cols = lines.clone().nth(0).unwrap().chars().count();
        let mut universe2: Vec<Vec<char>> = vec![vec!['0'; amount_cols]; amount_rows];
        for (i, line) in lines.enumerate() {
            for (j, c) in line.chars().enumerate() {
                universe2[i][j] = c;
            }
        }
        assert_eq!(universe2, universe);
    }
}
